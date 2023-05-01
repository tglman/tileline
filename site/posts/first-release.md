---
layout: post.liquid  
title: First release  
published_date: 2023-05-02 21:00:00.0 +0001
---

Tileline is a small project that I started in my free time (of free time), because I want to have a simple block(tile) visualization, that could have 
been fed with some arbitrary data.   

My final scope was to create something that would produce year tile lines fed with from external sources like source code management, website events, 
and what ever I could come up with, for build personal static blog/websites.   

So this is the first independent crate that goes in that direction.  

Not having much time to build this project myself, I'm doing a "early" open sourcing with the hope that other people jump in and play with it,
this to me feel like a good hobby project to help on and build something on top, and I hope other people are of the same opinion.

Even I do not have to much time to focus on this, I'm more than happy to do review of PR, publish releases, accept new random features,
code, documentation, website or what ever contribution come in.


Now that you know why, let's see what this do, this as today has 3 APIs, `tile`, `metadata_tile` and `year_tile`, that in the details they do:

- `tile` is a generic block visualization that receive an iterator of iterators that provide the data and create a block visualization, the data
consist in the "cell color", "border color", "Optional Link Label" and "Optional Link Destination"

- `metadata_tile` is like `tile` but it has an additional data source for labels to display around the block

- `year_tile` receive a year that drive the display of a year line, and a data source API, that ask the same data of the base line providing a specific
date

This is more or less all, there is some additional configuration to provide the sizes, border shapes and paddings.


Here some example output of the three features listed:

tile:

![tile](/images/tile.png)  

metadata_tile:

![metadata_tile](/images/metadata_tile.png)  

year_tile:

![year_tile](/images/year_tile.png)  

