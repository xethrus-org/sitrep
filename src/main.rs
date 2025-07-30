/*
use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(Dialog::text("Welcome to SitRep, ready to plan")
        .title("SitRep") 
        .button("Begin", |s| start_program(s, "lets"))
        .button("Another Time", |s| s.quit());
    }

    fn start_program(s: &mut Cursive, msg: &str) {
        s.pop_layer();
        s.add_layer(Dialog::text("select a country.")
            .title("Q1")
            .button("USA", |s| begin_national(s, "good choice"))
        //should try to make this a back rather than a quit
            .button("EXIT", |s| s.quit())
    }
    
    fn begin_national(s: &mut Cursive, msg: &str) {
        //take in a name of state, linear compare with a array or map of states
        s.pop_layer();
        s.add_layer(Dialog::text("select a state.")
            .title("Q2")
            .button("california", |s| begin_state(s))
        //should try to make this a back rather than a quit
            .button("EXIT", |s| s.quit())
    }
    
    fn select_nation(s: &mut Cursive) {
        let counter = AtomicUsize::new(1);
        s.menubar()
            .add_subtree(
                "Country",
                menu::Tree::new()
                    .leaf("New", move |s| {
                        let i = counter.fetch_add(1, Ordering::Relaxed);
                        let filename = format!("New {i}");
                        s.menubar()
                            .find_subtree("Country")
                            .unwrap()
                            .insert_leaf(0, country_name, |_| ());
                    }
        )
    }
    
    fn begin_state(s: &mut Cursive, msg: &str) {
        // button isnt assigning a value inside of s, it just is activating the logical jump
        
    }
    
    siv.run();
}
*/


// need to figure out if this cursive gui is what i want to use
//

