use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::core::event::Event;
use crate::lib::core::frame_context::FrameContext;

// Focus direction when navigating
#[derive(Clone, Copy)]
pub enum FocusDirection
{
    Up,
    Down,
    Left,
    Right
}

// View background
pub enum ViewBackground
{
    None,
    Sidebar,
    Backdrop,
    ShapeColor,
}

pub enum AlignSelf
{
    Auto,
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
    Baseline,
    SpaceBetween,
    SpaceAround,
}

// View visibility
pub enum Visibility
{
    Visible, // the view is visible
Invisible, // the view is invisible but still takes some space
Gone, // the view is invisible and doesn't take any space
}

// Position attribute behavior
pub enum PositionType
{
    Relative, // position attributes offset the view from the base layout
Absolute, // position attributes set the absolute coordinates of the view
}

// The animation to play when
// pushing / popping an activity or
// showing / hiding a view.
#[derive(Clone, Copy, PartialEq)]
pub enum TransitionAnimation
{
    Fade, // the old activity fades away and the new one fades in
SlideLeft, // the old activity slides out to the left and the new one slides in from the right
SlideRight, // inverted SLIDE_LEFT
}

// A View shape's shadow type
pub enum ShadowType
{
    None, // do not draw any shadow around the shape
Generic, // generic all-purpose shadow
Custom, // customized shadow (use the provided methods to tweak it)
}

pub type AutoAttributeHandler = fn();
pub type IntAttributeHandler = fn(i32);
pub type FloatAttributeHandler = fn(f64);
pub type StringAttributeHandler = fn(String);
pub type ColorAttributeHandler = fn(nanovg::Color);
pub type BoolAttributeHandler = fn(bool);
pub type FilePathAttributeHandler = fn(String);

pub type GenericEvent = Event<Rc<RefCell<Option<Box<dyn View>>>>> ;
pub type VoidEvent = Event<()>;

pub const AUTO: f32 = f32::NAN;

pub trait View {

    fn frame(&self, ctx: &FrameContext);
    fn get_default_focus(&self) -> Box<dyn View>;
    fn get_next_focus(&self, direction: FocusDirection, current_view: &dyn View) -> Box<dyn View>;
    fn on_focus_lost(&self);

    fn on_focus_gained(&self);

    fn describe(&self) -> String;

    fn get_view(&self, id: &str) -> Rc<RefCell<Option<Box<dyn View>>>>;

    fn get_parent(&self) -> Rc<RefCell<Option<Box<dyn View>>>>;
}