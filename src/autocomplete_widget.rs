use druid::text::{EditableText, TextStorage};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, RadioGroup, Scroll, TextBox, WidgetExt,
};
use druid::widget::{Dropdown, DROP};
use druid::{
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UnitPoint, UpdateCtx, Widget, WindowDesc,
};
use druid::im::Vector;

#[derive(Data, Clone, Lens)]
pub struct FuzzySearchData {
    pub(crate) word: String,
    pub(crate) words: Vector<String>,
    pub(crate) tolerance: usize,
}

pub struct AutoCompleteTextBox<T> {
    textbox: TextBox<T>,
}

impl<T: TextStorage + EditableText> AutoCompleteTextBox<T> {
    pub fn new() -> Self {
        AutoCompleteTextBox {
            textbox: TextBox::new(),
        }
    }
}

impl<T: TextStorage + EditableText> Widget<T> for AutoCompleteTextBox<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.textbox.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.textbox.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.textbox.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.textbox.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.textbox.paint(ctx, data, env);
    }
}
