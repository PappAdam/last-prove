use std::f32::consts::PI;

use nalgebra::Vector3;
use renderer::Renderer;

pub struct GameController {
    pub time: f32,
    //Time is in hours, from 0 to 24
    day_length: f32,
    //The time for 24 in-game hours to pass in seconds
    //Default is 120.

    //Any weather, or game event will come here.
}

impl GameController {
    pub fn add_time_elapsed(&mut self, time_elapsed: f32, renderer: &mut Renderer) {
        self.time += (time_elapsed / self.day_length) * 24.;
        if self.time > 24. {
            self.time -= 24.;
        }
        let sun_height = (self.time / (12. / PI)).sin();
        let sun_intensity = (sun_height.abs() * 3.).min(1.2);
        //Put this function in desmos.com to see the graph: min(abs(sin(x))*2.0, 1.5)

        let sun_direction =
            Vector3::new((self.time / (12. / PI)).cos(), sun_height, 0.2).normalize();

        if sun_height < 0. {
            renderer.data.push_const.sun_direction = sun_direction;
            renderer.data.push_const.sun_color = Vector3::new(0.9, 0.7, 0.7) * sun_intensity;
        } else {
            renderer.data.push_const.sun_direction = -sun_direction;
            renderer.data.push_const.sun_color = Vector3::new(0.5, 0.65, 0.87) * sun_intensity;
        }
    }
}

impl Default for GameController {
    fn default() -> Self {
        Self {
            time: Default::default(),
            day_length: 5.,
        }
    }
}
