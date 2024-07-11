
use gtk::{prelude::*, ComboBoxText};
use gtk::{ ApplicationWindow, Button,Grid,Box,Image};
use once_cell::sync::Lazy;
use rand::Rng;


pub const BUTTON_WIDTH:i32 = 30;
pub const BUTTON_HEIGHT:i32 = 30;
pub const WINDOW_WIDTH:i32 = 450;
pub const WINDOW_HEIGHT:i32 = 450;
pub struct MyUI{
     pub container_box: Box,
     pub buttons_grid : Vec<Button>,
     pub values_grid : Vec<u8>,
}
enum ClayerTurn {
    XTurn,
    OTurn,
}
enum Difficulty {
    Easy,
    Medium,
    Hard,
}


static mut WHO_S_TURN:ClayerTurn = ClayerTurn::XTurn;
pub static mut MY_UI_INSTANCE:Lazy<MyUI> = Lazy::new( || MyUI::new());
pub static mut GAME_FINISHED:bool = false;
static mut DIFFICULTY:Difficulty = Difficulty::Easy;
fn check_if_win() -> u8{
    //rows
    let mut won_row_0:bool = true;
    let mut won_row_1:bool = true;
    let mut won_row_2:bool = true;

    //collumns
    let mut won_coll_0:bool = true;
    let mut won_coll_1:bool = true;
    let mut won_coll_2:bool = true;

    //diagonals
    let mut won_diag_1:bool = true;
    let mut won_diag_2:bool = true;
    unsafe {
        for i in 0..2{
                //rows
                if MY_UI_INSTANCE.values_grid[i] == 0 || MY_UI_INSTANCE.values_grid[i] !=  MY_UI_INSTANCE.values_grid[i+1]{
                    won_row_0 = false;
                }
                if  MY_UI_INSTANCE.values_grid[i+3] == 0 || MY_UI_INSTANCE.values_grid[i+3] !=  MY_UI_INSTANCE.values_grid[i+4]{
                    won_row_1 = false;
                }
                if MY_UI_INSTANCE.values_grid[i+6] == 0 || MY_UI_INSTANCE.values_grid[i+6] !=  MY_UI_INSTANCE.values_grid[i+7]{
                    won_row_2 = false;
                }

                
                //collumns
                if MY_UI_INSTANCE.values_grid[i*3] == 0 || MY_UI_INSTANCE.values_grid[i*3] !=  MY_UI_INSTANCE.values_grid[(i+1)*3]{
                    won_coll_0 = false;
                }
                if MY_UI_INSTANCE.values_grid[i*3+ 1] == 0 || MY_UI_INSTANCE.values_grid[i*3+ 1] !=  MY_UI_INSTANCE.values_grid[(i+1)*3 + 1]{
                    won_coll_1 = false;
                }
                if MY_UI_INSTANCE.values_grid[i*3+ 2] == 0 || MY_UI_INSTANCE.values_grid[i*3+ 2] !=  MY_UI_INSTANCE.values_grid[(i+1)*3 + 2]{
                    won_coll_2 = false;
                }

                //diagonals
                if MY_UI_INSTANCE.values_grid[i*3 + i] == 0 || MY_UI_INSTANCE.values_grid[i*3 + i] !=  MY_UI_INSTANCE.values_grid[(i+1)*3 + i + 1]{
                    won_diag_1 = false;
                }
                if MY_UI_INSTANCE.values_grid[i*3 + (2-i)] == 0 || MY_UI_INSTANCE.values_grid[i*3 + (2-i)] !=  MY_UI_INSTANCE.values_grid[(i+1)*3 + (2-(i+1))]{
                    won_diag_2 = false;
                }
        
        }
        
        if won_row_1 || won_coll_1 || won_diag_1 || won_diag_2{
            return MY_UI_INSTANCE.values_grid[4];
        }
        else if  won_row_0 || won_coll_0 {
            return MY_UI_INSTANCE.values_grid[0];
        }
        else if won_row_2 ||  won_coll_2 {
            return MY_UI_INSTANCE.values_grid[8];
        }
    }


    return  0;
}

fn next_best_move() -> usize{
    unsafe {
        let mut dispo_moves:Vec<usize> = Vec::new();
        for  (i,value) in MY_UI_INSTANCE.values_grid.iter().enumerate(){
            if *value == 0u8{
                dispo_moves.push(i);
            }
        }
        let mut play_index: usize = 10;

        // attack block      
        let mut ai_collmns_counter: [u8; 3] = [0,0,0];
        let mut ai_collmns_index: [usize; 3] = [10,10,10];
        let mut ai_rows_counter: [u8; 3] = [0,0,0];
        let mut ai_rows_index: [usize; 3] = [10,10,10];
        let mut ai_diag_counter: [u8; 2] = [0,0];
        let mut ai_diag_index: [usize; 2] = [10,10];
        for i in 0..3{
            //rows ai counter
            if MY_UI_INSTANCE.values_grid[i] == 2u8{
                ai_collmns_counter[0] += 1;
            }else if MY_UI_INSTANCE.values_grid[i] == 0u8 {
                ai_collmns_index[0] = i;
            }
            if  MY_UI_INSTANCE.values_grid[i+3] == 2u8{
                ai_collmns_counter[1] += 1;
            }else if MY_UI_INSTANCE.values_grid[i+3] == 0u8{
                ai_collmns_index[1] = i+3;
            }
            if MY_UI_INSTANCE.values_grid[i+6] == 2u8{
                ai_collmns_counter[2] += 1;
            }else if MY_UI_INSTANCE.values_grid[i+6] == 0u8{
                ai_collmns_index[2] = i+6;
            }

            //collumns ai counter
            if MY_UI_INSTANCE.values_grid[i*3] == 2u8{
                ai_rows_counter[0] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3] == 0u8{
                ai_rows_index[0] = i*3;
            }
            if MY_UI_INSTANCE.values_grid[i*3+ 1] == 2u8{
                ai_rows_counter[1] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + 1] == 0u8{
                ai_rows_index[1] = i*3+ 1;
            }
            if MY_UI_INSTANCE.values_grid[i*3 + 2] == 2u8{
                ai_rows_counter[2] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + 2] == 0u8{
                ai_rows_index[2] = i*3+ 2;
            }
            
            //diagonals ai counter
            if MY_UI_INSTANCE.values_grid[i*3 + i] == 2u8{
                ai_diag_counter[0] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + i] == 0u8{
                ai_diag_index[0] = i*3 + i;
            }
            if MY_UI_INSTANCE.values_grid[i*3 + (2-i)] == 2u8 {
                ai_diag_counter[1] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + (2-i)] == 0u8{
                ai_diag_index[0] = i*3 + (2-i);
            }
        }
        for i in 0..3{
            if ai_collmns_counter[i] == 2{
                play_index = ai_collmns_index[i];
                break;
            }
            if ai_rows_counter[i] == 2{
                play_index = ai_rows_index[i];
                break;
            }
            if i < 2 && ai_diag_counter[i] == 2{
                play_index = ai_diag_index[i];
                break;
            }
        }
        if play_index < 9 {
            return play_index;
        }
        
        // defend block         
        let mut player_collmns_counter: [u8; 3] = [0,0,0];
        let mut player_collmns_index: [usize; 3] = [10,10,10];
        let mut player_rows_counter: [u8; 3] = [0,0,0];
        let mut player_rows_index: [usize; 3] = [10,10,10];
        let mut player_diag_counter: [u8; 2] = [0,0];
        let mut player_diag_index: [usize; 2] = [10,10];
        
        for i in 0..3{
            
            //rows player counter
            if MY_UI_INSTANCE.values_grid[i] == 1u8{
                player_collmns_counter[0] += 1;
            }else if MY_UI_INSTANCE.values_grid[i] == 0u8 {
                player_collmns_index[0] = i;
            }
            if  MY_UI_INSTANCE.values_grid[i+3] == 1u8{
                player_collmns_counter[1] += 1;
            }else if MY_UI_INSTANCE.values_grid[i+3] == 0u8{
                player_collmns_index[1] = i+3;
            }
            if MY_UI_INSTANCE.values_grid[i+6] == 1u8{
                player_collmns_counter[2] += 1;
            }else if MY_UI_INSTANCE.values_grid[i+6] == 0u8{
                player_collmns_index[2] = i+6;
            }

            //collumns player counter
            if MY_UI_INSTANCE.values_grid[i*3] == 1u8{
                player_rows_counter[0] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3] == 0u8{
                player_rows_index[0] = i*3;
            }
            if MY_UI_INSTANCE.values_grid[i*3+ 1] == 1u8{
                player_rows_counter[1] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + 1] == 0u8{
                player_rows_index[1] = i*3+ 1;
            }
            if MY_UI_INSTANCE.values_grid[i*3 + 2] == 1u8{
                player_rows_counter[2] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + 2] == 0u8{
                player_rows_index[2] = i*3+ 2;
            }

            //diagonals player counter
            if MY_UI_INSTANCE.values_grid[i*3 + i] == 1u8{
                player_diag_counter[0] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + i] == 0u8{
                player_diag_index[0] = i*3 + i;
            }
            if MY_UI_INSTANCE.values_grid[i*3 + (2-i)] == 1u8 {
                player_diag_counter[1] += 1;
            }else if MY_UI_INSTANCE.values_grid[i*3 + (2-i)] == 0u8{
                player_diag_index[1] = i*3 + (2-i);
            }
            

        }
        for i in 0..3{
            if player_collmns_counter[i] == 2{
                play_index = player_collmns_index[i];
                break;
            }
            if player_rows_counter[i] == 2{
                play_index = player_rows_index[i];
                break;
            }
            if i < 2 && player_diag_counter[i] == 2{
                play_index = player_diag_index[i];
                break;
            }
        }
        if play_index < 9 {
            return play_index;
        }

        for i in 0..3{
            if ai_collmns_counter[i] == 1 && player_collmns_counter[i] == 0{
                play_index = ai_collmns_index[i];
                break;
            }
            if ai_rows_counter[i] == 1 && ai_rows_counter[i] == 0{
                play_index = ai_rows_index[i];
                break;
            }
            if i < 2 && ai_diag_counter[i] == 1 && ai_diag_counter[i] == 0{
                play_index = ai_diag_index[i];
                break;
            }
        }
        if play_index < 9 {
            return play_index;
        }

        if dispo_moves.len() != 0{
            if play_index < 9 {
                return play_index;
            }
            let mut rng = rand::thread_rng();
            let n:usize =  rng.gen_range(0..(dispo_moves.len() as usize));
    
            return dispo_moves[n];
        }
    }
    return  10;
}
fn next_clever_move() -> usize{
    unsafe {
        let mut dispo_moves:Vec<usize> = Vec::new();
        for  (i,value) in MY_UI_INSTANCE.values_grid.iter().enumerate(){
            if *value == 0u8{
                dispo_moves.push(i);
            }
        }
        let mut play: usize = 10;
        for i in 0..2{
            //rows
            if MY_UI_INSTANCE.values_grid[i] != 0 && MY_UI_INSTANCE.values_grid[i] ==  MY_UI_INSTANCE.values_grid[i+1]{
                play = match i { 0 => 2, _ => 0};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }
            if  MY_UI_INSTANCE.values_grid[i+3] != 0 && MY_UI_INSTANCE.values_grid[i+3] == MY_UI_INSTANCE.values_grid[i+4]{
                play = match i { 0 => 5, _ => 3};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }
            if MY_UI_INSTANCE.values_grid[i+6] != 0 && MY_UI_INSTANCE.values_grid[i+6] ==  MY_UI_INSTANCE.values_grid[i+7]{
                play = match i { 0 => 8, _ => 6};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }

            
            //collumns
            if MY_UI_INSTANCE.values_grid[i*3] != 0 && MY_UI_INSTANCE.values_grid[i*3] == MY_UI_INSTANCE.values_grid[(i+1)*3]{
                play = match i { 0 => 6, _ => 0};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }
            if MY_UI_INSTANCE.values_grid[i*3+ 1] != 0 && MY_UI_INSTANCE.values_grid[i*3+ 1] == MY_UI_INSTANCE.values_grid[(i+1)*3 + 1]{
                play = match i { 0 => 7, _ => 1};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }
            if MY_UI_INSTANCE.values_grid[i*3+ 2] != 0 && MY_UI_INSTANCE.values_grid[i*3+ 2] == MY_UI_INSTANCE.values_grid[(i+1)*3 + 2]{
                play = match i { 0 => 8, _ => 2};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }

            //diagonals
            if MY_UI_INSTANCE.values_grid[i*3 + i] != 0 && MY_UI_INSTANCE.values_grid[i*3 + i] ==  MY_UI_INSTANCE.values_grid[(i+1)*3 + i + 1]{
                play = match i { 0 => 8, _ => 0};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }
            if MY_UI_INSTANCE.values_grid[i*3 + (2-i)] != 0 && MY_UI_INSTANCE.values_grid[i*3 + (2-i)] ==  MY_UI_INSTANCE.values_grid[(i+1)*3 + (2-(i+1))]{
                play = match i { 0 => 6, _ => 2};
                if MY_UI_INSTANCE.values_grid[play] == 0{
                    break;
                }
                else{
                    play = 10;
                }
            }
        }
        if dispo_moves.len() != 0{
            if play < 9 {
                return play;
            }
            let mut rng = rand::thread_rng();
            let n:usize =  rng.gen_range(0..(dispo_moves.len() as usize));
    
            return dispo_moves[n];
        }
    }
    return  10;
}
fn next_random_move() -> usize{
    unsafe {
        let mut dispo_moves:Vec<usize> = Vec::new();
        for  (i,value) in MY_UI_INSTANCE.values_grid.iter().enumerate(){
            if *value == 0u8{
                dispo_moves.push(i);
            }
        }
        if dispo_moves.len() != 0{
            let mut rng = rand::thread_rng();
            let n:usize =  rng.gen_range(0..(dispo_moves.len() as usize));
    
            return dispo_moves[n];
        }
    }
    return  10;
}
fn ai_playing() -> u8{
    let img;
    let op_img;
unsafe {
    op_img = match WHO_S_TURN {
        ClayerTurn::XTurn => {img = Image::builder().file("ressources/x.png").height_request(WINDOW_WIDTH/3).width_request(WINDOW_WIDTH/3).build();
        Some(&img)
        },
        ClayerTurn::OTurn=> {img = Image::builder().file("ressources/o.png").height_request(WINDOW_WIDTH/3).width_request(WINDOW_WIDTH/3).build();
        Some(&img)
        },
    };

    WHO_S_TURN = match WHO_S_TURN {
        ClayerTurn::XTurn => ClayerTurn::OTurn,
        ClayerTurn::OTurn => ClayerTurn::XTurn,
    };
    let next_move = match DIFFICULTY {
        Difficulty::Easy => next_random_move(),
        Difficulty::Medium => next_clever_move(),
        Difficulty::Hard => next_best_move(),
    };
    if next_move < 9 {
        MY_UI_INSTANCE.buttons_grid[next_move].set_image(op_img);
        MY_UI_INSTANCE.values_grid[next_move] = 2;
    }
    return check_if_win(); 
}

}
fn restart_button_click(_: &Button){
    unsafe {
        for  (i,_) in MY_UI_INSTANCE.values_grid.iter().enumerate(){
            MY_UI_INSTANCE.values_grid[i] = 0;
        }
        for  (i,_) in MY_UI_INSTANCE.buttons_grid.iter().enumerate(){
            MY_UI_INSTANCE.buttons_grid[i].set_image(Some(&Image::builder().height_request(WINDOW_WIDTH/3).width_request(WINDOW_WIDTH/3).build()));
        }

        GAME_FINISHED = false;
        WHO_S_TURN = ClayerTurn::XTurn;
    }
}
fn quit_button_click(_: &Button){
    unsafe{

        let window:ApplicationWindow =MY_UI_INSTANCE.container_box.parent().unwrap().unsafe_cast();
        window.application().unwrap().quit();
    }

}
fn playing_image_click(clicked_button:&Button){
    let img;
    let mut clicked_button_index : usize = 10;
    unsafe {
        if GAME_FINISHED == true{
            return;
        }
        for  (i,button) in MY_UI_INSTANCE.buttons_grid.iter().enumerate(){
            if button == clicked_button{
                clicked_button_index = i;
                break;
            }
        }
        if clicked_button_index > 8 ||  MY_UI_INSTANCE.values_grid[clicked_button_index] != 0{
            return;
        } 
        let op_img = match WHO_S_TURN {
            ClayerTurn::XTurn => {img = Image::builder().file("ressources/x.png").height_request(WINDOW_WIDTH/3).width_request(WINDOW_WIDTH/3).build();
            Some(&img)
            },
            ClayerTurn::OTurn=> {img = Image::builder().file("ressources/o.png").height_request(WINDOW_WIDTH/3).width_request(WINDOW_WIDTH/3).build();
            Some(&img)
            },
        };
        clicked_button.set_image(op_img);
        WHO_S_TURN = match WHO_S_TURN {
            ClayerTurn::XTurn => ClayerTurn::OTurn,
            ClayerTurn::OTurn => ClayerTurn::XTurn,
        };
        MY_UI_INSTANCE.values_grid[clicked_button_index] = 1;
        if check_if_win() == 1{
            println!("Player won !");
            GAME_FINISHED = true;
        }
        else if ai_playing() == 2{
            println!("AI won !");
            GAME_FINISHED = true;
        }
    }
}

fn change_difficulty(switch : &ComboBoxText){
    println!("active changed ! {} choosed !",switch.active_text().unwrap());
    let choosed_difficulty = switch.active_text().unwrap().to_string();
    unsafe {
        DIFFICULTY = match choosed_difficulty.as_str() {
            "Easy" => Difficulty::Easy,
            "Medium" => Difficulty::Medium,
            _ => Difficulty::Hard,
        };
    }
}
impl MyUI {
    pub fn new() -> MyUI{
        let backbutton = Button::builder().label("Restart").width_request(BUTTON_WIDTH).height_request(BUTTON_HEIGHT).expand(false).build();
        let quitbutton: Button = Button::builder().label("Quit").width_request(BUTTON_WIDTH).height_request(BUTTON_HEIGHT).expand(false).build();
        let difficulty_switch = ComboBoxText::builder().width_request(100).build();
        difficulty_switch.append_text("Easy");
        difficulty_switch.append_text("Medium");
        difficulty_switch.append_text("Hard");
        difficulty_switch.set_active(Some(0));
        difficulty_switch.connect_active_notify(change_difficulty);
        let buttons_box = Box::builder().orientation(gtk::Orientation::Horizontal).child(&backbutton).child(&difficulty_switch).child(&quitbutton).halign(gtk::Align::Center).spacing(WINDOW_WIDTH/4).expand(false).valign(gtk::Align::End).build();
        
        let mut images =Vec::new();
        let mut image_buttons: Vec<Button> =Vec::new();
        let mut values_grid : Vec<u8> = Vec::new();
        for _ in 0..9{
            let img = Image::builder().height_request(WINDOW_WIDTH/3).width_request(WINDOW_WIDTH/3).build();
            let button = Button::builder().image(&img).width_request(WINDOW_WIDTH/3).height_request(WINDOW_WIDTH/3).expand(false).build();
            button.connect_clicked(playing_image_click);
            image_buttons.push(button);
            values_grid.push(0);
            images.push(img);
        }
        
        let grid :Grid = Grid::builder().expand(false).orientation(gtk::Orientation::Vertical).build();

        for i in 0..3{
            let grid_collumn:Grid = Grid::builder().expand(false).orientation(gtk::Orientation::Horizontal).build();
            for j in 0..3{
                grid_collumn.add(&image_buttons[j+i*3]);
            }
            grid.add(&grid_collumn);
        }

        let container_box = Box::builder().orientation(gtk::Orientation::Vertical).child(&grid).child(&buttons_box).build();

        //events
        backbutton.connect_clicked(restart_button_click);
        quitbutton.connect_clicked(quit_button_click);

        MyUI {container_box : container_box,buttons_grid : image_buttons ,values_grid : values_grid }
    }
}