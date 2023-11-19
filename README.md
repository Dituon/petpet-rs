# petpet-rs

## Intro

**[Try Online](https://petpet.d2n.moe/)**

根据模板生成图像, 详见 [Petpet标准](https://github.com/Dituon/petpet)
- [Java 实现](https://github.com/Dituon/petpet)
- [JS 实现](https://github.com/Dituon/petpet-js)

## todo

##### perf

- [ ] thread pool
- [ ] cache surface
- [ ] GPU filter

#### template

**Text**

- [ ] `Text.*`

**Avatar**

- [x] `Avatar.type`

- [x] `Avatar.pos`
- [x] `Avatar.posType`

- [ ] `Avatar.crop`
- [ ] `Avatar.cropType`

- [ ] `Avatar.style`
- [ ] `Avatar.filter`
- [x] `Avatar.fit`

- [ ] `Avatar.round`
- [x] `Avatar.rotate`
- [x] `Avatar.origin`
- [x] `Avatar.avatarOnTop`

- [x] `Avatar.angle`
- [x] `Avatar.opacity`


**Background**

- [x] size
- [ ] color

#### Core

- [x] decode GIF
- [ ] encode GIF

#### Server

- [ ] get
- [ ] post
- [ ] form-data