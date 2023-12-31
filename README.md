# petpet-rs

## Intro

**[Try Online](https://petpet.d2n.moe/)**

根据模板生成图像, 详见 [Petpet标准](https://github.com/Dituon/petpet)
- [Java 实现](https://github.com/Dituon/petpet)
- [JS 实现](https://github.com/Dituon/petpet-js)

## todo

##### perf

- [x] thread pool
- [ ] cache surface
- [x] GPU filter
- [ ] local filter
- [x] config file
- [ ] mix fonts

#### template

**Text**

**TextData**

- [x] `TextData.text`
- [x] `TextData.pos`
- [x] `TextData.angle`
- [x] `TextData.color`
- [x] `TextData.font`
- [x] `TextData.size`
- [x] `TextData.align`
- [x] `TextData.wrap.BREAK`
- [x] `TextData.wrap.ZOOM`
- [x] `TextData.style`
- [ ] `TextData.position`
- [x] `TextData.origin`
- [x] `TextData.strokeColor`
- [x] `TextData.strokeSize`
- [ ] `TextData.greedy`

**Avatar**

- [x] `Avatar.type`
- [x] `Avatar.pos`
- [x] `Avatar.posType`
- [x] `Avatar.crop`
- [x] `Avatar.cropType`
- [x] `Avatar.style.MIRROR`
- [x] `Avatar.style.FLIP`
- [x] `Avatar.style.GRAY`
- [x] `Avatar.style.BINARIZATION`
- [x] `Avatar.filter.SWIRL`
- [x] `Avatar.filter.BULGE`
- [x] `Avatar.filter.SWIM`
- [x] `Avatar.filter.BLUR`
- [x] `Avatar.filter.CONTRAST`
- [x] `Avatar.filter.HSB`
- [x] `Avatar.filter.HALFTONE`
- [x] `Avatar.filter.DOT_SCREEN`
- [x] `Avatar.filter.NOISE`
- [x] `Avatar.filter.DENOISE`
- [x] `Avatar.filter.OIL`
- [x] `Avatar.fit`
- [x] `Avatar.round`
- [x] `Avatar.rotate`
- [x] `Avatar.origin`
- [x] `Avatar.avatarOnTop`
- [x] `Avatar.angle`
- [x] `Avatar.opacity`


**Background**

- [x] `Background.size`
- [x] `Background.color`
- [x] `Background.length`

#### Core

- [x] decode GIF
- [x] encode GIF

#### Server

- [x] get
- [x] post
- [ ] form-data

#### Feature

- `Avatar.crop` not working when `Avatar.fit: COVER`
- size variable not working when `Avatar.posType: DEFORM`
- Mixing of matrices is inexact
