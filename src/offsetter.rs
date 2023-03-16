use iced::{touch, Event};
use iced_native::event;
use iced_native::layout::{self, Layout};
use iced_native::mouse;
use iced_native::overlay;
use iced_native::renderer;
use iced_native::widget::horizontal_space;
use iced_native::widget::tree::{self, Tree};
use iced_native::{Clipboard, Element, Length, Point, Rectangle, Shell, Size, Widget};

use ouroboros::self_referencing;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::ops::Deref;

#[allow(missing_debug_implementations)]
pub struct Offsetter<'a, Message, Renderer> {
    size: Size,
    scroll_by: f32,
    view: Box<dyn Fn(f32) -> Element<'a, Message, Renderer> + 'a>,
    content: RefCell<Content<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Offsetter<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    pub fn new(
        size: Size,
        scroll_by: f32,
        view: impl Fn(f32) -> Element<'a, Message, Renderer> + 'a,
    ) -> Self {
        Self {
            size,
            scroll_by,
            view: Box::new(view),
            content: RefCell::new(Content {
                offset: 0.0,
                layout: layout::Node::new(Size::ZERO),
                element: Element::new(horizontal_space(Length::Units(0))),
            }),
        }
    }
}

struct Content<'a, Message, Renderer> {
    offset: f32,
    layout: layout::Node,
    element: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Content<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn update(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        size: Size,
        offset: f32,
        view: &dyn Fn(f32) -> Element<'a, Message, Renderer>,
    ) {
        self.offset = offset;
        self.element = view(self.offset);

        tree.diff(&self.element);

        self.layout = self
            .element
            .as_widget()
            .layout(renderer, &layout::Limits::new(Size::ZERO, size));
    }

    fn resolve<R, T>(
        &mut self,
        tree: &mut Tree,
        renderer: R,
        layout: Layout<'_>,
        size: Size,
        offset: f32,
        view: &dyn Fn(f32) -> Element<'a, Message, Renderer>,
        f: impl FnOnce(&mut Tree, R, Layout<'_>, &mut Element<'a, Message, Renderer>) -> T,
    ) -> T
    where
        R: Deref<Target = Renderer>,
    {
        self.update(tree, renderer.deref(), size, offset, view);

        let content_layout = Layout::with_offset(layout.position() - Point::ORIGIN, &self.layout);

        f(tree, renderer, content_layout, &mut self.element)
    }
}

struct State {
    tree: RefCell<Tree>,
    scroller: ScrollerState,
}

pub struct ScrollerState {
    offset_pixels: f32,
    height: f32,
}
impl ScrollerState {
    pub fn new(height: f32) -> Self {
        Self {
            offset_pixels: 0.0,
            height,
        }
    }
    pub fn scroll(&mut self, delta_y: f32) {
        // println!("delta_y: {}", delta_y);
        self.offset_pixels = (self.offset_pixels - delta_y).max(0.0).min(self.height);
        // println!("offset: {}", self.offset_pixels);
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Offsetter<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            tree: RefCell::new(Tree::empty()),
            scroller: ScrollerState::new(self.size.height),
        })
    }

    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        layout::Node::new(limits.max())
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let bounds = layout.bounds();
        let is_mouse_over = bounds.contains(cursor_position);
        let state = tree.state.downcast_mut::<State>();
        let mut content = self.content.borrow_mut();

        if is_mouse_over {
            match event {
                Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                    match delta {
                        mouse::ScrollDelta::Lines { y, .. } => {
                            // println!("lines: {}", y);
                            state.scroller.scroll(y * self.scroll_by);
                        }
                        mouse::ScrollDelta::Pixels { y, .. } => {
                            // println!("pixels: {}", y);
                            state.scroller.scroll(y);
                        }
                    }

                    // notify_on_scroll(state, on_scroll, bounds, content_bounds, shell);

                    return event::Status::Captured;
                }
                Event::Touch(event) => {
                    match event {
                        touch::Event::FingerPressed { .. } => {
                            // state.scroll_box_touched_at = Some(cursor_position);
                        }
                        touch::Event::FingerMoved { .. } => {
                            // if let Some(scroll_box_touched_at) = state.scroll_box_touched_at {
                            //     let delta = cursor_position.y - scroll_box_touched_at.y;

                            //     state.scroll(delta, bounds, content_bounds);

                            //     state.scroll_box_touched_at = Some(cursor_position);

                            //     // notify_on_scroll(state, on_scroll, bounds, content_bounds, shell);
                            // }
                        }
                        touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. } => {
                            // state.scroll_box_touched_at = None;
                        }
                    }

                    return event::Status::Captured;
                }
                _ => {}
            }
        }

        content.resolve(
            &mut state.tree.borrow_mut(),
            renderer,
            layout,
            self.size,
            state.scroller.offset_pixels,
            &self.view,
            |tree, renderer, layout, element| {
                element.as_widget_mut().on_event(
                    tree,
                    event,
                    layout,
                    cursor_position,
                    renderer,
                    clipboard,
                    shell,
                )
            },
        );

        event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let mut content = self.content.borrow_mut();

        content.resolve(
            &mut state.tree.borrow_mut(),
            renderer,
            layout,
            self.size,
            state.scroller.offset_pixels,
            &self.view,
            |tree, renderer, layout, element| {
                element.as_widget().draw(
                    tree,
                    renderer,
                    theme,
                    style,
                    layout,
                    cursor_position,
                    viewport,
                )
            },
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let mut content = self.content.borrow_mut();

        content.resolve(
            &mut state.tree.borrow_mut(),
            renderer,
            layout,
            self.size,
            state.scroller.offset_pixels,
            &self.view,
            |tree, renderer, layout, element| {
                element.as_widget().mouse_interaction(
                    tree,
                    layout,
                    cursor_position,
                    viewport,
                    renderer,
                )
            },
        )
    }

    fn overlay<'b>(
        &'b self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let state = tree.state.downcast_ref::<State>();

        let overlay = OverlayBuilder {
            content: self.content.borrow_mut(),
            tree: state.tree.borrow_mut(),
            types: PhantomData,
            overlay_builder: |content, tree| {
                content.update(
                    tree,
                    renderer,
                    layout.bounds().size(),
                    state.scroller.offset_pixels,
                    &self.view,
                );

                let content_layout =
                    Layout::with_offset(layout.position() - Point::ORIGIN, &content.layout);

                content
                    .element
                    .as_widget()
                    .overlay(tree, content_layout, renderer)
            },
        }
        .build();

        let has_overlay =
            overlay.with_overlay(|overlay| overlay.as_ref().map(overlay::Element::position));

        has_overlay.map(|position| overlay::Element::new(position, Box::new(overlay)))
    }
}

impl<'a, Message, Renderer> From<Offsetter<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + 'a,
    Message: 'a,
{
    fn from(responsive: Offsetter<'a, Message, Renderer>) -> Self {
        Self::new(responsive)
    }
}

#[self_referencing]
struct Overlay<'a, 'b, Message, Renderer> {
    content: RefMut<'a, Content<'b, Message, Renderer>>,
    tree: RefMut<'a, Tree>,
    types: PhantomData<Message>,

    #[borrows(mut content, mut tree)]
    #[covariant]
    overlay: Option<overlay::Element<'this, Message, Renderer>>,
}

impl<'a, 'b, Message, Renderer> Overlay<'a, 'b, Message, Renderer> {
    fn with_overlay_maybe<T>(
        &self,
        f: impl FnOnce(&overlay::Element<'_, Message, Renderer>) -> T,
    ) -> Option<T> {
        self.borrow_overlay().as_ref().map(f)
    }

    fn with_overlay_mut_maybe<T>(
        &mut self,
        f: impl FnOnce(&mut overlay::Element<'_, Message, Renderer>) -> T,
    ) -> Option<T> {
        self.with_overlay_mut(|overlay| overlay.as_mut().map(f))
    }
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
    for Overlay<'a, 'b, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
        self.with_overlay_maybe(|overlay| {
            let vector = position - overlay.position();

            overlay.layout(renderer, bounds).translate(vector)
        })
        .unwrap_or_default()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
    ) {
        let _ = self.with_overlay_maybe(|overlay| {
            overlay.draw(renderer, theme, style, layout, cursor_position);
        });
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.with_overlay_maybe(|overlay| {
            overlay.mouse_interaction(layout, cursor_position, viewport, renderer)
        })
        .unwrap_or_default()
    }

    fn on_event(
        &mut self,
        event: iced_native::Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.with_overlay_mut_maybe(|overlay| {
            overlay.on_event(event, layout, cursor_position, renderer, clipboard, shell)
        })
        .unwrap_or(iced_native::event::Status::Ignored)
    }
}
