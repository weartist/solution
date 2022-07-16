
trait WaitingTime {
    fn waiting_time(light: SignalLight) -> u8;
}

enum SignalLight {
    Red,
    Yellow,
    Green,
}

impl WaitingTime for SignalLight {
    fn waiting_time(light: SignalLight) -> u8 {
        match light {
            SignalLight::Red => 120,
            SignalLight::Yellow => 5,
            SignalLight::Green => 60,
        }
    }
}

fn main() {
    println!("the red light duration is {} seconds.",SignalLight::waiting_time(SignalLight::Red));
    println!("the yellow light duration is {} seconds.",SignalLight::waiting_time(SignalLight::Yellow));
    println!("the greed light duration is {} seconds.",SignalLight::waiting_time(SignalLight::Green));
}