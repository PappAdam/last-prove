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
        //Time goes from 0 to 24
        self.time += (time_elapsed / self.day_length) * 24.;
        if self.time > 24. {
            self.time -= 24.;
        }

        //Calculating sun height, and intensity based on time
        //A cosine function, so that 24 in time = 360 degrees rotation.
        //Peak at 12 hour, is 0 at 6 and 18.
        let sun_height = -((self.time / (12. / PI)).cos());
        let sun_intensity = (sun_height.abs() * 0.4).min(0.3);
        //See graph at desmos.com: \min(\operatorname{abs}(\cos\left(\frac{x\ \cdot\ \pi}{12}\right))*3.0,1.2)
        //Where x equals time.

        //Calculating sun direction based time.
        let sun_direction = Vector3::new(
            (self.time / (12. / PI)).sin(),
            //See graph at desmos.com: \sin(\frac{x\ \cdot\ \pi}{12})
            sun_height,
            (self.time / (12. / PI)).sin() / 2.,
            //See graph at desmos.com: \frac{\sin(\frac{x\ \cdot\ \pi}{12})}{2}
        )
        .normalize();

        //Separatin day and night time
        if sun_height > 0. {
            //Daytime
            renderer.data.push_const.sun_direction = -sun_direction;
            renderer.data.push_const.sun_color = Vector3::new(0.7, 0.7, 0.7) * sun_intensity;
        } else {
            //Nighttime
            renderer.data.push_const.sun_direction = sun_direction;
            renderer.data.push_const.sun_color =
                Vector3::new(0.5, 0.65, 0.87) * sun_intensity * 0.6;
        }
    }
}

impl GameController {
    pub fn init(renderer: &mut Renderer) -> Self {
        let mut game_controller = Self {
            time: 11.,
            day_length: 10.,
        };
        game_controller.add_time_elapsed(0., renderer);
        game_controller
    }
}
