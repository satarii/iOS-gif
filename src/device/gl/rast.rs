// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use r = super::super::rast;
use super::gl;


pub fn bind_primitive(p: r::Primitive) {
    gl::FrontFace(match p.front_face {
        r::Cw => gl::CW,
        r::Ccw => gl::CCW,
    });

    let (gl_draw, gl_offset) = match p.method {
        r::Point => (gl::POINT, gl::POLYGON_OFFSET_POINT),
        r::Line(width) => {
            gl::LineWidth(width);
            (gl::LINE, gl::POLYGON_OFFSET_LINE)
        },
        r::Fill(front, back) => {
            if front == r::DrawFront && back == r::DrawBack {
                gl::Disable(gl::CULL_FACE);
            }else {
                gl::Enable(gl::CULL_FACE);
                gl::CullFace(match (front, back) {
                    (r::DrawFront, r::CullBack) => gl::BACK,
                    (r::CullFront, r::DrawBack) => gl::FRONT,
                    (r::CullFront, r::CullBack) => gl::FRONT_AND_BACK,
                    _ => unreachable!(),
                });
            }
            (gl::FILL, gl::POLYGON_OFFSET_FILL)
        },
    };

    gl::PolygonMode(gl::FRONT_AND_BACK, gl_draw);

    match p.offset {
        r::Offset(factor, units) => {
            gl::Enable(gl_offset);
            gl::PolygonOffset(factor, units as gl::types::GLfloat);
        },
        r::NoOffset => gl::Disable(gl_offset),
    }
}


fn map_comparison(cmp: r::Comparison) -> gl::types::GLenum {
    match cmp {
        r::Comparison(r::NoLess, r::NoEqual, r::NoGreater) => gl::NEVER,
        r::Comparison(r::Less,   r::NoEqual, r::NoGreater) => gl::LESS,
        r::Comparison(r::NoLess, r::Equal,   r::NoGreater) => gl::EQUAL,
        r::Comparison(r::Less,   r::Equal,   r::NoGreater) => gl::LEQUAL,
        r::Comparison(r::NoLess, r::NoEqual, r::Greater)   => gl::GREATER,
        r::Comparison(r::Less,   r::NoEqual, r::Greater)   => gl::NOTEQUAL,
        r::Comparison(r::NoLess, r::Equal,   r::Greater)   => gl::GEQUAL,
        r::Comparison(r::Less,   r::Equal,   r::Greater)   => gl::ALWAYS,
    }
}

pub fn bind_depth(depth: Option<r::Depth>) {
    match depth {
        Some(d) => {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(map_comparison(d.fun));
            gl::DepthMask(if d.write {gl::TRUE} else {gl::FALSE});
        },
        None => gl::Disable(gl::DEPTH_TEST),
    }
}

