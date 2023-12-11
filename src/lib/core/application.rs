use std::cell::RefCell;
use std::rc::Rc;
use log::info;
use crate::lib::core::input::{ControllerState, InputManager};
use crate::lib::core::platform::Platform;
use crate::lib::core::time::{get_cpu_time_usec, Timestamp};
use crate::lib::platform::glfw::platform::GlfwPlatform;

static BUTTON_REPEAT_DELAY: i32 = 15;
static BUTTON_REPEAT_CADENCY: i32 = 5;

pub struct Application {
    platform: Rc<RefCell<Box<dyn Platform>>>,
    title: String,
    window_width: u32,
    window_height: u32,
    quit_requested: bool,
    old_controller_state: ControllerState,
}

impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Application{
            platform: Rc::new(RefCell::new(Box::new(GlfwPlatform::new(title, width,  height)))),
            title: title.into(),
            window_width: width,
            window_height: height,
            quit_requested: false,
            old_controller_state: ControllerState::new(),
        }
    }

    pub fn main_loop(&mut self) -> bool {
        // info!("main_loop");
        // std::thread::sleep(std::time::Duration::from_millis(1000));

        // Main loop callback
        if !self.platform.borrow_mut().main_loop_iteration() || self.quit_requested {
            self.exit();
            return false
        }

        // Input
        let input_manager_rc = self.platform.borrow_mut().get_input_manager();
        let input_manager = input_manager_rc.borrow_mut();
        let controller_state = input_manager.get_controller_state();

        // info!("controller_state: {:#?}", controller_state.buttons);

        // Trigger controller events
        let mut any_button_pressed = false;
        let mut repeating = false;
        let mut button_press_time: Timestamp = 0;
        let mut repeating_button_timer: i32 = 0;

        for i in 0..controller_state.buttons.len() {
            if controller_state.buttons[i] {
                any_button_pressed = true;
                repeating = repeating_button_timer > BUTTON_REPEAT_DELAY && repeating_button_timer % BUTTON_REPEAT_CADENCY == 0;
                info!("button {}, pressed", i);
                if !self.old_controller_state.buttons[i] || repeating {
                    // self.on_controller_button_pressed(i, repeating);
                }
            }

            if controller_state.buttons[i] != self.old_controller_state.buttons[i] {
                repeating_button_timer = 0;
                button_press_time = 0;
            }
        }

        if any_button_pressed && get_cpu_time_usec() - button_press_time > 1000 {
            button_press_time = get_cpu_time_usec();
            repeating_button_timer +=1; // Increased once every ~1ms
        }

        self.old_controller_state = controller_state;

        true
    }

    pub fn exit(&mut self) {
        info!("Exiting...");
        self.clear();
    }

    pub fn clear(&mut self) {
        // for activity in self.activities_stack {
        //     activity.borrow_mut().will_appear(true);
        // }
        //
        // self.activities_stack.clear();
    }
}