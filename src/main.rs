mod morning;
mod evening;
mod noon;
mod night;

fn main() {
    morning::wake_up();
    morning::coffee();
    noon::go_to("12:00");
    noon::go_home("18:00");
}
