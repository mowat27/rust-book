mod banner;
mod boxlist;
mod limits;
mod mybox;
mod rclist;

fn main() {
    boxlist::run();
    mybox::run();
    rclist::run();
    limits::run();
}
