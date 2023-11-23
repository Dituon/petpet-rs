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
- [ ] GPU filter

#### template

**Text**

- [ ] `Text.*`

**Avatar**

- [x] `Avatar.type`

- [x] `Avatar.pos`
- [x] `Avatar.posType`

- [x] `Avatar.crop`
- [x] `Avatar.cropType`

- [ ] `Avatar.style`
- [ ] `Avatar.filter`
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

- [ ] get
- [x] post
- [ ] form-data