use std::sync::{Arc, Mutex};
use wai_bindgen_rust::Handle;

wai_bindgen_rust::export!("rustfft.wai");

struct Rustfft {}
impl rustfft::Rustfft for Rustfft {}
pub struct Complex(original::num_complex::Complex64);

impl Complex {
    pub fn new(state: original::num_complex::Complex64) -> Self {
        Self(state)
    }
}

impl rustfft::Complex for Complex {
    fn new_f64(re: f64, im: f64) -> Handle<Complex> {
        Handle::new(Complex::new(original::num_complex::Complex64::new(re, im)))
    }

    fn i(&self) -> Handle<Complex> {
        Handle::new(Complex::new(original::num_complex::Complex64::i()))
    }
}

impl From<Complex> for original::num_complex::Complex64 {
    fn from(complex: Complex) -> Self {
        Self {
            re: complex.0.re,
            im: complex.0.im,
        }
    }
}

pub struct InnerFft(Arc<dyn original::Fft<f64> + 'static>);

impl InnerFft {
    pub fn new(state: Arc<dyn original::Fft<f64> + 'static>) -> Self {
        Self(state)
    }
}

impl rustfft::InnerFft for InnerFft {
    fn process_with_scratch(&self, buffer: Vec<Handle<Complex>>, scratch: Vec<Handle<Complex>>) {
        let mut buffer_vec = buffer
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        let mut scratch_vec = scratch
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        self.0
            .process_with_scratch(&mut buffer_vec, &mut scratch_vec)
    }

    fn process_outofplace_with_scratch(
        &self,
        input: Vec<Handle<Complex>>,
        output: Vec<Handle<Complex>>,
        scratch: Vec<Handle<Complex>>,
    ) {
        let mut input_vec = input
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        let mut output_vec = output
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        let mut scratch_vec = scratch
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        self.0
            .process_outofplace_with_scratch(&mut input_vec, &mut output_vec, &mut scratch_vec)
    }

    fn get_inplace_scratch_len(&self) -> u32 {
        self.0.get_inplace_scratch_len() as u32
    }

    fn get_outofplace_scratch_len(&self) -> u32 {
        self.0.get_outofplace_scratch_len() as u32
    }

    fn process(&self, buffer: Vec<Handle<Complex>>) {
        let mut buffer_vec = buffer
            .iter()
            .map(|complex| complex.0)
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
    pub fn new(state: original::FftPlanner<f64>) -> Self {
        Self(Mutex::new(state))
    }
}

impl rustfft::FftPlanner for FftPlanner {
    fn new_fft_planner() -> Handle<FftPlanner> {
        Handle::new(FftPlanner::new(original::FftPlanner::new()))
    }
    fn plan_fft(&self, len: u32, direction: rustfft::FftDirection) -> Handle<InnerFft> {
        let mut planner = self.0.lock().expect("Mutex was poisioned");
        Handle::new(InnerFft::new(
            planner.plan_fft(len as usize, direction.into()),
        ))
    }

    fn plan_fft_forward(&self, len: u32) -> wai_bindgen_rust::Handle<crate::InnerFft> {
        let mut planner = self.0.lock().expect("Mutex was poisioned");
        Handle::new(InnerFft::new(
            planner.plan_fft(len as usize, original::FftDirection::Forward),
        ))
    }

    fn plan_fft_inverse(&self, len: u32) -> wai_bindgen_rust::Handle<crate::InnerFft> {
        let mut planner = self.0.lock().expect("Mutex was poisioned");
        Handle::new(InnerFft::new(
            planner.plan_fft(len as usize, original::FftDirection::Forward),
        ))
    }
}

pub trait Algo {
    fn algo_len(&self) -> u32;
    fn algo_fft_direction(&self) -> rustfft::FftDirection;
    fn algo_process_with_scratch(
        &self,
        buffer: Vec<Handle<Complex>>,
        scratch: Vec<Handle<Complex>>,
    );
    fn algo_process_outofplace_with_scratch(
        &self,
        input: Vec<Handle<Complex>>,
        output: Vec<Handle<Complex>>,
        scratch: Vec<Handle<Complex>>,
    );
    fn algo_get_inplace_scratch_len(&self) -> u32;
    fn algo_get_outofplace_scratch_len(&self) -> u32;
    fn algo_process(&self, buffer: Vec<Handle<Complex>>);
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

    fn algo_process_with_scratch(
        &self,
        buffer: Vec<Handle<Complex>>,
        scratch: Vec<Handle<Complex>>,
    ) {
        let mut buffer_vec = buffer
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        let mut scratch_vec = scratch
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        self.process_with_scratch(&mut buffer_vec, &mut scratch_vec)
    }

    fn algo_process_outofplace_with_scratch(
        &self,
        input: Vec<Handle<Complex>>,
        output: Vec<Handle<Complex>>,
        scratch: Vec<Handle<Complex>>,
    ) {
        let mut input_vec = input
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        let mut output_vec = output
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        let mut scratch_vec = scratch
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();

        self.process_outofplace_with_scratch(&mut input_vec, &mut output_vec, &mut scratch_vec)
    }

    fn algo_get_inplace_scratch_len(&self) -> u32 {
        self.get_inplace_scratch_len() as u32
    }

    fn algo_get_outofplace_scratch_len(&self) -> u32 {
        self.get_outofplace_scratch_len() as u32
    }

    fn algo_process(&self, buffer: Vec<Handle<Complex>>) {
        let mut buffer_vec = buffer
            .iter()
            .map(|complex| complex.0)
            .collect::<Vec<original::num_complex::Complex64>>();
        self.process(&mut buffer_vec)
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
        width_fft: wai_bindgen_rust::Handle<crate::InnerFft>,
        height_fft: wai_bindgen_rust::Handle<crate::InnerFft>,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
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
        width_fft: wai_bindgen_rust::Handle<crate::InnerFft>,
        height_fft: wai_bindgen_rust::Handle<crate::InnerFft>,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        let width_fft = &width_fft.0;
        let height_fft = &height_fft.0;
        Handle::new(Algorithm::new(original::algorithm::MixedRadix::new(
            width_fft.to_owned(),
            height_fft.to_owned(),
        )))
    }

    fn new_mixed_radix_small(
        width_fft: wai_bindgen_rust::Handle<crate::InnerFft>,
        height_fft: wai_bindgen_rust::Handle<crate::InnerFft>,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        let width_fft = &width_fft.0;
        let height_fft = &height_fft.0;
        Handle::new(Algorithm::new(original::algorithm::MixedRadixSmall::new(
            width_fft.to_owned(),
            height_fft.to_owned(),
        )))
    }

    fn new_raders_algorithm(
        inner_fft: wai_bindgen_rust::Handle<crate::InnerFft>,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        let inner_fft = &inner_fft.0;
        Handle::new(Algorithm::new(original::algorithm::RadersAlgorithm::new(
            inner_fft.to_owned(),
        )))
    }

    fn new_radix3(
        len: u32,
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(original::algorithm::Radix3::new(
            len as usize,
            direction.into(),
        )))
    }

    fn new_radix4(
        len: u32,
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
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

    fn new_butterfly3(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly3::new(direction.into()),
        ))
    }

    fn new_butterfly4(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly4::new(direction.into()),
        ))
    }

    fn new_butterfly5(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly5::new(direction.into()),
        ))
    }

    fn new_butterfly6(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly6::new(direction.into()),
        ))
    }

    fn new_butterfly7(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly7::new(direction.into()),
        ))
    }

    fn new_butterfly8(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly8::new(direction.into()),
        ))
    }

    fn new_butterfly9(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly9::new(direction.into()),
        ))
    }

    fn new_butterfly11(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly11::new(direction.into()),
        ))
    }

    fn new_butterfly13(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly13::new(direction.into()),
        ))
    }

    fn new_butterfly16(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly16::new(direction.into()),
        ))
    }

    fn new_butterfly17(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly17::new(direction.into()),
        ))
    }

    fn new_butterfly19(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly19::new(direction.into()),
        ))
    }

    fn new_butterfly23(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly23::new(direction.into()),
        ))
    }

    fn new_butterfly27(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly27::new(direction.into()),
        ))
    }

    fn new_butterfly29(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly29::new(direction.into()),
        ))
    }

    fn new_butterfly31(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
        Handle::new(Algorithm::new(
            original::algorithm::butterflies::Butterfly31::new(direction.into()),
        ))
    }

    fn new_butterfly32(
        direction: rustfft::FftDirection,
    ) -> wai_bindgen_rust::Handle<crate::Algorithm> {
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

    fn process_with_scratch(&self, buffer: Vec<Handle<Complex>>, scratch: Vec<Handle<Complex>>) {
        let algo = self.0.lock().expect("Mutex was Poisioned");
        algo.algo_process_with_scratch(buffer, scratch)
    }

    fn process_outofplace_with_scratch(
        &self,
        input: Vec<Handle<Complex>>,
        output: Vec<Handle<Complex>>,
        scratch: Vec<Handle<Complex>>,
    ) {
        let algo = self.0.lock().expect("Mutex was Poisioned");
        algo.algo_process_outofplace_with_scratch(input, output, scratch)
    }

    fn get_inplace_scratch_len(&self) -> u32 {
        let algo = self.0.lock().expect("Mutex was Poisioned");
        algo.algo_get_inplace_scratch_len()
    }

    fn get_outofplace_scratch_len(&self) -> u32 {
        let algo = self.0.lock().expect("Mutex was Poisioned");
        algo.algo_get_outofplace_scratch_len()
    }

    fn process(&self, buffer: Vec<Handle<Complex>>) {
        let algo = self.0.lock().expect("Mutex was Poisioned");
        algo.algo_process(buffer)
    }
}
