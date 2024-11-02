# **PineGUI**

（[中文文档](README_zh-cn.md)）

**A Rust embedded GUI library, which is based on embedded-graphics that do the rendering content works.**

## - [x] platform

Provides platform-related traits that users can implement for their boards.
e.g. timer for timing, etc.
Provide platform-related traits that users can implement for their boards.

**"Screen" is the type that implements the `DrawTarget` trait in embedded-graphics**.

## - [x] layout

**Used to handle the positional relationships between different elements and to provide layout methods such as flex, grid, etc.**

## - [x] style

Style an element (equivalent to writing CSS).

## - [x] event

For handling input events (mouse, key...)

The user binds an event handler (e.g. onclick, etc.) to an element, and when an input event is received, the user calls the method above event dispatcher, and the event dispatcher dispatches the event to the element bound to the event handler (executes the handler) (and determines whether the event belongs to the element or not.) (we also need to determine if the event belongs to the element, for example, a touch event requires that the element be touched in order to trigger the handler)

## pinegui

Provide definitions for components, states, etc.

### component

Provide a `Component` trait with methods such as new (which creates the component), view (which writes the main logic of the component and returns a type that implements the `Draw` trait).
Components can also have hooks, such as create, destroy, and so on.

### state

The `State` struct stores a reference to the component that uses the state.

You can call the set() method on a state to change its value, and the value(comp_ref) method to get the value of the state (the value method takes an `&impl Component` type).

When set() is called, the component that uses this state is re-rendered (first determining if it is on the screen (visible or not)).

## animation

Animation library, providing some ready-made animations (pan, rotate, etc.).

## components

Component library, provides some ready-made components (buttons, textboxes, etc.).
