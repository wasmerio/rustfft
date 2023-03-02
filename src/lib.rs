use std::sync::{Arc, Mutex};
use wai_bindgen_rust::Handle;

wai_bindgen_rust::export!("rustfft.wai");

use rustfft::Complex;

struct Rustfft {}
impl rustfft::Rustfft for Rustfft {}

impl From<Complex> for original::num_complex::Complex64 {
    fn from(complex: Complex) -> Self {
        let Complex { re, im } = complex;
        Self { re, im }
    }
}

impl From<original::num_complex::Complex64> for Complex {
    fn from(complex: original::num_complex::Complex64) -> Self {
        let original::num_complex::Complex64 { re, im } = complex;
        Self { re, im }
    }
}

pub struct InnerFft(Arc<dyn original::Fft<f64>>);

impl InnerFft {
    pub fn new(state: Arc<dyn original::Fft<f64> + 'static>) -> Self {
        Self(state)
    }
}

impl rustfft::InnerFft for InnerFft {
    fn process(&self, buffer: Vec<Complex>) {
        let mut buffer_vec = buffer
            .iter()
            .map(|complex| original::num_complex::Complex64::from(*complex))
            .collect::<Vec<original::num_complex::Complex64>>();
        self.0.process(&mut buffer_vec)
    }
}

impl From<rustfft::FftDirection> for original::FftDirection {
    fn from(direction: rustfft::FftDirection) -> Self {
        match direction {
            rustfft::FftDirection::Forward => original::FftDirection::Forward,
            rustfft::FftDirection::Inverse => original::FftDirection::Inverse,
        }
    }
}

impl From<original::FftDirection> for rustfft::FftDirection {
    fn from(direction: original::FftDirection) -> Self {
        match direction {
            original::FftDirection::Forward => rustfft::FftDirection::Forward,
            original::FftDirection::Inverse => rustfft::FftDirection::Inverse,
        }
    }
}

pub struct FftPlanner(Mutex<original::FftPlanner<f64>>);

impl FftPlanner {
    pub fn new_from_state(state: original::FftPlanner<f64>) -> Self {
        Self(Mutex::new(state))
    }
}

impl rustfft::FftPlanner for FftPlanner {
    fn new() -> Handle<FftPlanner> {
        Handle::new(FftPlanner::new_from_state(original::FftPlanner::new()))
    }
    fn plan_fft(&self, len: u32, direction: rustfft::FftDirection) -> Handle<InnerFft> {
        let mut planner = self.0.lock().expect("Mutex was poisioned");
        Handle::new(InnerFft::new(
            planner.plan_fft(len as usize, direction.into()),
        ))
    }

    fn plan_fft_forward(&self, len: u32) -> wai_bindgen_rust::Handle<InnerFft> {
        let mut planner = self.0.lock().expect("Mutex was poisioned");
        Handle::new(InnerFft::new(
            planner.plan_fft(len as usize, original::FftDirection::Forward),
        ))
    }

    fn plan_fft_inverse(&self, len: u32) -> wai_bindgen_rust::Handle<InnerFft> {
        let mut planner = self.0.lock().expect("Mutex was poisioned");
        Handle::new(InnerFft::new(
            planner.plan_fft(len as usize, original::FftDirection::Forward),
        ))
    }
}

pub trait Algo {
    fn algo_len(&self) -> u32;
    fn algo_fft_direction(&self) -> rustfft::FftDirection;
    fn algo_compute(&self, buffer: Vec<Complex>) -> Vec<Complex>;
}

impl<T> Algo for T
where
    T: original::Fft<f64> + original::Direction + original::Length,
{
    fn algo_len(&self) -> u32 {
        self.len() as u32
    }

    fn algo_fft_direction(&self) -> rustfft::FftDirection {
        self.fft_direction().into()
    }

    fn algo_compute(&self, buffer: Vec<Complex>) -> Vec<Complex> {
        let mut buffer_vec = buffer
            .iter()
            .map(|complex| original::num_complex::Complex64::from(*complex))
            .collect::<Vec<original::num_complex::Complex64>>();

        self.process(&mut buffer_vec);

        buffer_vec
            .iter()
            .map(|complex| Complex::from(*complex))
            .collect::<Vec<Complex>>()
    }
}

pub struct Algorithm(Mutex<Box<dyn Algo>>);

impl Algorithm {
    pub fn new(state: impl Algo + 'static) -> Self {
        Self(Mutex::new(Box::new(state)))
    }
}

impl rustfft::Algorithm for Algorithm {
    fn new_bluesteins_algorithm(len: u32, inner_fft: Handle<InnerFft>) -> Handle<Algorithm> {
        let inner_fft = &inner_fft.0;
        Handle::new(Algorithm::new(
            original::algorithm::BluesteinsAlgorithm::new(len as usize, inner_fft.to_owned()),
        ))
    }

    fn new_dft(len: u32, direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(original::algorithm::Dft::new(
            len as usize,
            direction.into(),
        )))
    }

    fn new_good_thomas_algorithm(
        width_fft: Handle<InnerFft>,
        height_fft: Handle<InnerFft>,
    ) -> Handle<Algorithm> {
        let width_fft = &width_fft.0;
        let height_fft = &height_fft.0;
        Handle::new(Algorithm::new(
            original::algorithm::GoodThomasAlgorithm::new(
                width_fft.to_owned(),
                height_fft.to_owned(),
            ),
        ))
    }

    fn new_good_thomas_algorithm_small(
        width_fft: Handle<InnerFft>,
        height_fft: Handle<InnerFft>,
    ) -> Handle<Algorithm> {
        let width_fft = &width_fft.0;
        let height_fft = &height_fft.0;
        Handle::new(Algorithm::new(
            original::algorithm::GoodThomasAlgorithmSmall::new(
                width_fft.to_owned(),
                height_fft.to_owned(),
            ),
        ))
    }

    fn new_mixed_radix(
        width_fft: Handle<InnerFft>,
        height_fft: Handle<InnerFft>,
    ) -> Handle<Algorithm> {
        let width_fft = &width_fft.0;
        let height_fft = &height_fft.0;
        Handle::new(Algorithm::new(original::algorithm::MixedRadix::new(
            width_fft.to_owned(),
            height_fft.to_owned(),
        )))
    }

    fn new_mixed_radix_small(
        width_fft: Handle<InnerFft>,
        height_fft: Handle<InnerFft>,
    ) -> Handle<Algorithm> {
        let width_fft = &width_fft.0;
        let height_fft = &height_fft.0;
        Handle::new(Algorithm::new(original::algorithm::MixedRadixSmall::new(
            width_fft.to_owned(),
            height_fft.to_owned(),
        )))
    }

    fn new_raders_algorithm(inner_fft: Handle<InnerFft>) -> Handle<Algorithm> {
        let inner_fft = &inner_fft.0;
        Handle::new(Algorithm::new(original::algorithm::RadersAlgorithm::new(
            inner_fft.to_owned(),
        )))
    }

    fn new_radix3(len: u32, direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(original::algorithm::Radix3::new(
            len as usize,
            direction.into(),
        )))
    }

    fn new_radix4(len: u32, direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(original::algorithm::Radix4::new(
            len as usize,
            direction.into(),
        )))
    }

    fn new_butterfly1(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly1::new(direction.into()),
        ))
    }

    fn new_butterfly2(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly2::new(direction.into()),
        ))
    }

    fn new_butterfly3(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly3::new(direction.into()),
        ))
    }

    fn new_butterfly4(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly4::new(direction.into()),
        ))
    }

    fn new_butterfly5(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly5::new(direction.into()),
        ))
    }

    fn new_butterfly6(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly6::new(direction.into()),
        ))
    }

    fn new_butterfly7(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly7::new(direction.into()),
        ))
    }

    fn new_butterfly8(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly8::new(direction.into()),
        ))
    }

    fn new_butterfly9(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly9::new(direction.into()),
        ))
    }

    fn new_butterfly11(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly11::new(direction.into()),
        ))
    }

    fn new_butterfly13(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly13::new(direction.into()),
        ))
    }

    fn new_butterfly16(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly16::new(direction.into()),
        ))
    }

    fn new_butterfly17(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly17::new(direction.into()),
        ))
    }

    fn new_butterfly19(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly19::new(direction.into()),
        ))
    }

    fn new_butterfly23(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly23::new(direction.into()),
        ))
    }

    fn new_butterfly27(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly27::new(direction.into()),
        ))
    }

    fn new_butterfly29(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly29::new(direction.into()),
        ))
    }

    fn new_butterfly31(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly31::new(direction.into()),
        ))
    }

    fn new_butterfly32(direction: rustfft::FftDirection) -> Handle<Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly32::new(direction.into()),
        ))
    }

    fn len(&self) -> u32 {
        let algo = self.0.lock().expect("Mutex was poisioned");
        algo.algo_len()
    }

    fn fft_direction(&self) -> rustfft::FftDirection {
        let algo = self.0.lock().expect("Mutex was Poisioned");
        algo.algo_fft_direction()
    }

    fn compute(&self, signal: Vec<Complex>) -> Vec<Complex> {
        let algo = self.0.lock().expect("Mutex was Poisioned");
        algo.algo_compute(signal)
    }
}
