---
layout: default.liquid
title: Tileline
---

Tileline is a simple Rust crate to create SVG block(tile) visualization, a easy way to understand what it means is just looking at the logo, 
that was generated using the lib.

![Tileline](/logo.png "Tileline")

For more details about the feature checkout the first release post

- News
{% for post in collections.posts.pages %}
    - [{{ post.title }}]({{ post.permalink }})
{% endfor %}
- [Source](https://github.com/tglman/tileline)
- [crates.io](https://crates.io/tileline)
