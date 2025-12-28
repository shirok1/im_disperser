use std::collections::VecDeque;
use vizia_plug::vizia::{prelude::*, vg};

pub enum WaveformViewEvent {
    AddSample1(f32),
    AddSample2(f32),
}

pub struct WaveformView {
    buffer1: VecDeque<f32>,
    buffer2: VecDeque<f32>,
    max_samples: usize,
}

impl WaveformView {
    pub fn new<L1, L2>(
        cx: &mut Context,
        lens1: L1,
        lens2: L2,
        max_samples: usize,
    ) -> Handle<'_, Self>
    where
        L1: Lens<Target = f32>,
        L2: Lens<Target = f32>,
    {
        Self {
            buffer1: VecDeque::from(vec![0.0; max_samples]),
            buffer2: VecDeque::from(vec![0.0; max_samples]),
            max_samples,
        }
        .build(cx, |cx| {
            Binding::new(cx, lens1, |cx, sample_lens1| {
                let sample = sample_lens1.get(cx);
                cx.emit(WaveformViewEvent::AddSample1(sample));
            });
            Binding::new(cx, lens2, |cx, sample_lens2| {
                let sample = sample_lens2.get(cx);
                cx.emit(WaveformViewEvent::AddSample2(sample));
            });
        })
    }

    fn push_sample1(&mut self, sample: f32) {
        if self.buffer1.len() >= self.max_samples {
            self.buffer1.pop_front();
        }
        self.buffer1.push_back(sample);
    }

    fn push_sample2(&mut self, sample: f32) {
        if self.buffer2.len() >= self.max_samples {
            self.buffer2.pop_front();
        }
        self.buffer2.push_back(sample);
    }
}

impl View for WaveformView {
    fn element(&self) -> Option<&'static str> {
        Some("waveform-view")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|waveform_event, _| match waveform_event {
            WaveformViewEvent::AddSample1(sample1) => {
                self.push_sample1(*sample1);
                cx.needs_redraw();
            }
            WaveformViewEvent::AddSample2(sample2) => {
                self.push_sample2(*sample2);
                cx.needs_redraw();
            }
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let background_color = cx.background_color();
        let stroke_color = cx.font_color();
        let stroke_width = cx.border_width().max(1.5);

        let mut bg_paint = vg::Paint::default();
        bg_paint.set_color(background_color);
        let rect = vg::Rect::from_xywh(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.draw_rect(&rect, &bg_paint);

        let mid_y = bounds.y + bounds.h / 2.0;
        let half_h = bounds.h / 2.0;
        let x_step = bounds.w / (self.max_samples as f32 - 1.0);

        let mut wave_path = vg::Path::new();

        //// buffer1
        // Top half
        let mut first = true;
        for (i, &sample) in self.buffer1.iter().enumerate() {
            let x = bounds.x + i as f32 * x_step;
            let sample_height = sample.abs().clamp(0.0, 1.0) * half_h;
            let y = mid_y - sample_height;
            if first {
                wave_path.move_to((x, y));
                first = false;
            } else {
                wave_path.line_to((x, y));
            }
        }

        // Bottom half, in reverse
        for (i, &sample) in self.buffer1.iter().enumerate().rev() {
            let x = bounds.x + i as f32 * x_step;
            let sample_height = sample.abs().clamp(0.0, 1.0) * half_h;
            let y = mid_y + sample_height;
            wave_path.line_to((x, y));
        }

        wave_path.close();

        // 渲染填充区域
        let mut fill_paint = vg::Paint::default();
        let fill_color = stroke_color;
        fill_paint.set_dither(true);
        fill_paint.set_color(fill_color);
        fill_paint.set_alpha(114);
        fill_paint.set_style(vg::PaintStyle::Fill);
        fill_paint.set_anti_alias(true);
        canvas.draw_path(&wave_path, &fill_paint);

        // 渲染轮廓线
        let mut stroke_paint = vg::Paint::default();
        stroke_paint.set_color(stroke_color);
        stroke_paint.set_stroke_width(stroke_width);
        stroke_paint.set_style(vg::PaintStyle::Stroke);
        stroke_paint.set_stroke_cap(vg::PaintCap::Round);
        stroke_paint.set_stroke_join(vg::PaintJoin::Round);
        stroke_paint.set_anti_alias(true);
        canvas.draw_path(&wave_path, &stroke_paint);

        //// buffer2
        // Top half
        let mut first = true;
        for (i, &sample) in self.buffer2.iter().enumerate() {
            let x = bounds.x + i as f32 * x_step;
            let sample_height = sample.abs().clamp(0.0, 1.0) * half_h;
            let y = mid_y - sample_height;
            if first {
                wave_path.move_to((x, y));
                first = false;
            } else {
                wave_path.line_to((x, y));
            }
        }

        // Bottom half, in reverse
        for (i, &sample) in self.buffer2.iter().enumerate().rev() {
            let x = bounds.x + i as f32 * x_step;
            let sample_height = sample.abs().clamp(0.0, 1.0) * half_h;
            let y = mid_y + sample_height;
            wave_path.line_to((x, y));
        }

        wave_path.close();

        // 渲染轮廓线
        let mut stroke_paint = vg::Paint::default();
        stroke_paint.set_color(Color::palegreen());
        stroke_paint.set_alpha_f(0.5);
        stroke_paint.set_stroke_width(stroke_width);
        stroke_paint.set_style(vg::PaintStyle::Stroke);
        stroke_paint.set_stroke_cap(vg::PaintCap::Round);
        stroke_paint.set_stroke_join(vg::PaintJoin::Round);
        stroke_paint.set_anti_alias(true);
        canvas.draw_path(&wave_path, &stroke_paint);

        // 绘制零位基准线
        let mut center_line = vg::Path::new();
        center_line.move_to((bounds.x, mid_y));
        center_line.line_to((bounds.x + bounds.w, mid_y));
        let mut center_paint = vg::Paint::default();
        center_paint.set_color(Color::rgba(255, 255, 255, 20)); // 很淡的白色
        center_paint.set_stroke_width(1.0);
        center_paint.set_style(vg::PaintStyle::Stroke);
        canvas.draw_path(&center_line, &center_paint);
    }
}
