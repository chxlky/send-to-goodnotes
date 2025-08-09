use iced::advanced::layout::{Limits, Node};
use iced::advanced::widget::Operation;
use iced::advanced::widget::tree::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::{Element, Event, Rectangle};
use iced::{event, mouse};

pub struct OutsideCommit<'a, M: Clone> {
    content: Element<'a, M>,
    editing: bool,
    on_outside: Option<M>,
}

impl<'a, M: Clone> OutsideCommit<'a, M> {
    pub fn new(content: Element<'a, M>, editing: bool, on_outside: Option<M>) -> Self {
        Self {
            content,
            editing,
            on_outside,
        }
    }
}

impl<'a, M: Clone + 'a> Widget<M, iced::Theme, iced::Renderer> for OutsideCommit<'a, M> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        if tree.children.is_empty() {
            tree.children.push(Tree::new(&self.content));
        } else {
            tree.children[0].diff(&self.content);
        }
    }

    fn size(&self) -> iced::Size<iced::Length> {
        self.content.as_widget().size()
    }

    fn layout(&self, tree: &mut Tree, renderer: &iced::Renderer, limits: &Limits) -> Node {
        if tree.children.is_empty() {
            tree.children.push(Tree::new(&self.content));
        }
        self.content
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if tree.children.is_empty() {
            tree.children.push(Tree::new(&self.content));
        }
        self.content
            .as_widget()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, M>,
        viewport: &Rectangle,
    ) -> event::Status {
        if tree.children.is_empty() {
            tree.children.push(Tree::new(&self.content));
        }
        let status = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event
            && self.editing
            && matches!(status, event::Status::Ignored)
            && let Some(msg) = self.on_outside.clone()
        {
            shell.publish(msg);
            return event::Status::Captured;
        }
        status
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &iced::Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if tree.children.is_empty() {
            return;
        }
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if tree.children.is_empty() {
            return mouse::Interaction::Idle;
        }
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }
}

impl<'a, M: Clone + 'a> From<OutsideCommit<'a, M>> for Element<'a, M> {
    fn from(w: OutsideCommit<'a, M>) -> Self {
        Element::new(w)
    }
}
