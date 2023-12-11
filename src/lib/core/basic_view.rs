use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::lib::core::animation::Animatable;
use crate::lib::core::audio;
use crate::lib::core::theme::Theme;
use crate::lib::core::time::Timestamp;
use crate::lib::core::view::{AutoAttributeHandler, BoolAttributeHandler, ColorAttributeHandler, FilePathAttributeHandler, FloatAttributeHandler, FocusDirection, GenericEvent, ShadowType, StringAttributeHandler, View, ViewBackground, Visibility};
use yoga::{Node as YGNode};

// Superclass for all the other views
// Lifecycle of a view is :
//   new -> [willAppear -> willDisappear] -> delete
//
// Users have do to the new, the rest of the lifecycle is taken
// care of by the library
//
// willAppear and willDisappear can be called zero or multiple times
// before deletion (in case of a TabLayout for instance)
pub struct BasicView {
    background: ViewBackground,
    highlight_alpha: Animatable,
    highlight_padding: f32,
    highlight_corner_radius: f32,
    click_alpha: Animatable, // animated between 0 and 1
    highlight_shaking: bool,
    highlight_shake_start: Timestamp,
    highlight_shake_direction: FocusDirection,
    highlight_shake_amplitude: f32,
    fade_in: bool, // is the fade in animation running?
    in_fade_animation: bool, // is any fade animation running?
    theme_override: Option<Theme>,
    hidden: bool,
    focusable: bool,
    focus_sound: audio::Sound,
    hide_highlight_background: bool,
    detached: bool,
    detached_origin_x: f32,
    detached_origin_y: f32,
    translation_x: f32,
    translation_y: f32,
    wireframe_enabled: bool,

    actions: Vec<crate::lib::core::actions::Action>,

    /**
     * Parent user data, typically the index of the view
     * in the internal layout structure
     */
    parent_userdata: bool,

    culled: bool, // will be culled by the parent Box, if any

    bound_documents: Vec<String>,

    auto_attributes: HashMap<String, AutoAttributeHandler>,
    percentage_attributes: HashMap<String, FloatAttributeHandler>,
    float_attributes: HashMap<String, FloatAttributeHandler>,
    string_attributes: HashMap<String, StringAttributeHandler>,
    color_attributes: HashMap<String, ColorAttributeHandler>,
    bool_attributes: HashMap<String, BoolAttributeHandler>,
    file_path_attributes: HashMap<String, FilePathAttributeHandler>,

    known_attributes: HashSet<String>,

    maximum_allowed_xml_elements: u32,

    line_color: nanovg::Color,
    line_top: f32,
    line_right: f32,
    line_bottom: f32,
    line_left: f32,

    visibility: Visibility,

    background_color: nanovg::Color,
    border_color: nanovg::Color,
    border_thickness: f32,
    corner_radius: f32,
    shadow_type: ShadowType,
    show_shadow: bool,

    custom_focus_by_id: HashMap<FocusDirection, String>,
    custom_focus_by_ptr: HashMap<FocusDirection, Rc<RefCell<Box<dyn View>>>>,

    collapse_state: Animatable,

    focused: bool,

    focus_event: GenericEvent,

    yg_node: YGNode,

    id: String,

    alpha: Animatable,
}