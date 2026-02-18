slint::include_modules!();
use std::{sync::Mutex};
use std::time::Duration;
use slint::invoke_from_event_loop;
use tokio::runtime::{Runtime};
use std::sync::Arc;
use slint::Timer;
use std::rc::Rc;
use std::cell::RefCell;

struct Entity{
    entity_pos:Arc<Mutex<[f32;2]>>,
    speed_inc:Arc<Mutex<f32>>,
    is_reatched:Arc<Mutex<bool>>,
}
impl Entity{
    fn run(&mut self,game: slint::Weak<MainWindow>)
    {
        let entity_pos1 = self.entity_pos.clone();
        let is_reatched_gate_cln = self.is_reatched.clone();

        let speed_inc_ptr = self.speed_inc.clone();
        std::thread::spawn(move ||{
            let runtime = Runtime::new().unwrap();
            let is_reatched_gate_cln_cln = is_reatched_gate_cln.clone();
            runtime.block_on(async move {
                loop 
                {
                    let weak_game = game.clone();
                    let entity_pos_clone = entity_pos1.clone();
                    let is_reatched_gate_cln_cln_cln = is_reatched_gate_cln_cln.clone();
                    let speed_inc_ptr_ptr_clone = speed_inc_ptr.clone();
                    invoke_from_event_loop(move || {
                        
                        if let Some(game) = weak_game.upgrade(){
                            *entity_pos_clone.lock().unwrap() = [game.get_g_x(),game.get_g_y()];
                            if *is_reatched_gate_cln_cln_cln.lock().unwrap() == true
                            {
                                println!("Reatched Generating new Point for the target !");
                                let mut sp = speed_inc_ptr_ptr_clone.lock().unwrap(); 
                                //update speed
                                if *sp <= 400.0{
                                    *sp = *sp + 0.1;
                                }
                                *is_reatched_gate_cln_cln_cln.lock().unwrap() = false;
                                let window_size = game.window().size();
                                game.set_t_x(rand::random_range(-(window_size.width as f32 / 2.0)..(window_size.width as f32 / 2.0)));
                                game.set_t_y(rand::random_range(-(window_size.height as f32 / 2.0)..(window_size.height as f32 / 2.0)));
                                }else
                                {
                                    println!("Waiting Entity to reatch target ! Actual speed {}",*speed_inc_ptr_ptr_clone.lock().unwrap());
                                    //*is_reatched_gate_cln_cln_cln.lock().unwrap() = true;
                                    //move entity once reatched the target set *is_reatched_gate_cln_cln_cln.lock().unwrap() = true;
                                    //so it generate a new target and so on
                                    let weak_game = game.clone_strong().as_weak();
                                    let is_reached = Rc::new(RefCell::new(false));
                                        Timer::single_shot(std::time::Duration::from_millis(16), move || {
                                            if let Some(game) = weak_game.upgrade() {
                                            let direction_vector: (f32,f32) = (game.get_t_x() - game.get_g_x() as f32, game.get_t_y() - game.get_g_y() as f32);
                                            let distance = (direction_vector.0.powf(2.0) + direction_vector.1.powf(2.0)).sqrt();
                                            if distance != 0.0 {
                                                game.set_g_x(game.get_g_x() + (direction_vector.0 / distance) * *speed_inc_ptr_ptr_clone.lock().unwrap());
                                                game.set_g_y(game.get_g_y() + (direction_vector.1 / distance) * *speed_inc_ptr_ptr_clone.lock().unwrap());
                                            }
                                            if distance < *speed_inc_ptr_ptr_clone.lock().unwrap() {
                                                game.set_g_x(game.get_t_x());
                                                game.set_g_y(game.get_t_y());
                                                *is_reatched_gate_cln_cln_cln.lock().unwrap() = true;
                                            } 
                                            Timer::single_shot(std::time::Duration::from_millis(0), || {});
                                        }   
                                    }); 
                                }
                        }
                    }).unwrap();
                    tokio::time::sleep(Duration::from_millis(0)).await;
    
                }
            });
        });
    }

    
}

fn main() {
    let gameapp = MainWindow::new().unwrap();
    
    let mut entity = Entity{
        entity_pos: Arc::new(Mutex::new([0.0,0.0])),
        speed_inc:Arc::new(Mutex::new(0.1)),
        is_reatched:Arc::new(Mutex::new(true))
    };

    entity.run(gameapp.as_weak());


    gameapp.run().unwrap();
}
