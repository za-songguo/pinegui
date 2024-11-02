# PineGUI

## 基于 embedded-graphics 把内容渲染到屏幕上

## - [x] platform

提供一些平台相关的 traits，由用户为他们的板子实现这些 traits
例如用于定时的 timer 等

**“屏幕”就是实现了 embedded-graphics 中 `DrawTarget` trait 的类型**

## - [x] layout

用于处理不同元素之间的位置关系，同时提供一些布局方式，比如 flex, grid 等

## - [x] style

用于给元素添加样式（相当于写 CSS）

## - [x] event

用于处理输入事件（触摸、按键等）

用户先在元素上绑定 event handler（如 onclick 等），接收到输入事件后用户调用 event dispatcher 上面的方法，event dispatcher 就会把事件派发给绑定了 event handler 的元素（执行 handler）（还要判断事件是不是属于这个元素的，比如触摸事件要求触摸到了这个元素才能触发 handler）

## - [x] pinegui

提供组件、state 等的定义

### - [x] component

提供一个 `Component` trait，上面有一些方法，比如 new（创建这个组件），view（里面写组件的主逻辑，返回一个实现了 `Draw` trait 的类型）
组件还可以有 hook，比如 create, destroy 等

### - [x] state

`State` struct 里存储用到这个 state 的组件的引用

可以调用 state 上的 set() 方法来修改 state 的值，调用 value(comp_ref) 方法来获取 state 的值（value 方法里面接收一个`&impl Component` 类型）

调用 set 方法时，会将用到这个 state 中的 component 重新渲染（先判断是否在屏幕上（是否能看到））

## - [x] pinegui-animation

动画库，提供一些现成的动画（平移，旋转等）

## - [x] pinegui-components

组件库，提供一些现成的组件（按钮，文本框等）
