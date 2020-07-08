mod banner;
mod boxlist;
mod limits;
mod mybox;
mod rc_refcell_list;
mod rclist;

fn main() {
    boxlist::run();
    mybox::run();
    rclist::run();
    limits::run();
    rc_refcell_list::run();
}
