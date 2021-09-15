use {
    super::font,
    conquer_once::spin::OnceCell,
    core::{
        convert::{TryFrom, TryInto},
        ops::{Index, IndexMut},
        slice,
    },
    os_units::Bytes,
    rgb::RGB8,
    spinning_top::{const_spinlock, Spinlock, SpinlockGuard},
    syscalls::ScreenInfo,
    vek::Vec2,
    x86_64::VirtAddr,
};

static VRAM: Spinlock<Vram> = const_spinlock(Vram);
static SCREEN_INFO: OnceCell<ScreenInfo> = OnceCell::uninit();
static FRAME_BUFFER: OnceCell<VirtAddr> = OnceCell::uninit();

const BPP: u32 = 32;

pub(super) fn init(screen_info: ScreenInfo) {
    init_frame_buffer(&screen_info);
    init_info(screen_info);
    clear_screen();
}

pub(super) fn scroll_up() {
    lock().scroll_up();
}

pub(super) fn set_color(coord: Vec2<usize>, color: RGB8) {
    let (x, y) = coord.into_tuple();

    lock()[y][x] = color.into();
}

pub(super) fn resolution() -> Vec2<u32> {
    Vec2 {
        x: info().resolution_x(),
        y: info().resolution_y(),
    }
}

fn lock() -> SpinlockGuard<'static, Vram> {
    VRAM.try_lock()
        .expect("Failed to acquire the lock of `VRAM`")
}

fn init_frame_buffer(screen_info: &ScreenInfo) {
    let len = screen_info.scan_line_width() * screen_info.resolution_y() * 4;
    let len = Bytes::new(len.try_into().unwrap());

    unsafe {
        let virt = syscalls::map_memory(screen_info.frame_buffer(), len);

        FRAME_BUFFER
            .try_init_once(|| virt)
            .expect("Failed to initialize `FRAME_BUFFER`");
    }
}

fn init_info(screen_info: ScreenInfo) {
    SCREEN_INFO
        .try_init_once(|| screen_info)
        .expect("Failed to initialize `SCREEN_INFO`");
}

fn clear_screen() {
    lock().clear();
}

fn frame_buffer() -> VirtAddr {
    *FRAME_BUFFER
        .try_get()
        .expect("`FRAME_BUFFER` is not initialized.")
}

fn info<'a>() -> &'a ScreenInfo {
    SCREEN_INFO.try_get().expect("`INFO` is not initialized.")
}

struct Vram;
impl Vram {
    fn clear(&mut self) {
        let (x, y): (usize, usize) = resolution().as_().into_tuple();
        for y in 0..y {
            for x in 0..x {
                self[y][x] = Bgr::default();
            }
        }
    }

    fn scroll_up(&mut self) {
        let fh: usize = font::HEIGHT.try_into().unwrap();
        let (w, h): (usize, usize) = resolution().as_().into_tuple();
        let lc = h / fh;
        let log_bottom = fh * (lc - 1);

        for x in 0..w {
            for y in 0..log_bottom {
                self[y][x] = self[y + fh][x];
            }

            for y in log_bottom..h {
                self[y][x] = Bgr::default();
            }
        }
    }
}
impl Index<usize> for Vram {
    type Output = [Bgr];

    fn index(&self, index: usize) -> &Self::Output {
        let p = frame_buffer() + index * usize::try_from(resolution().x * BPP / 8).unwrap();

        unsafe { slice::from_raw_parts(p.as_ptr(), resolution().x.try_into().unwrap()) }
    }
}
impl IndexMut<usize> for Vram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let p = frame_buffer() + index * usize::try_from(resolution().x * BPP / 8).unwrap();

        unsafe { slice::from_raw_parts_mut(p.as_mut_ptr(), resolution().x.try_into().unwrap()) }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
struct Bgr {
    b: u8,
    g: u8,
    r: u8,
    _alpha: u8,
}
impl From<RGB8> for Bgr {
    fn from(rgb: RGB8) -> Self {
        Self {
            b: rgb.b,
            g: rgb.g,
            r: rgb.r,
            _alpha: 0,
        }
    }
}
