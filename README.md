# Composing Fractals

In this repository, I copied the code from the paper [Composing Fractals](http://web.cecs.pdx.edu/~mpj/pubs/composing-fractals.html)
by Mark P. Jones into the Main file in the haskell folder and then re-implemented it in Rust.
I only copied the ASCII-based renderer. Here's what it looks like:

```
''''''''''''''''''''''''''''""""""""""""""""""""""""""""""""""""""""""""'''''''
''''''''''''''''''''''''""""""""""""""""""""""""""""""""""""""""""""""""""""'''
''''''''''''''''''''""""""""""""""""""""""""""""""""""""""""""""""""""""""""""'
'''''''''''''''''"""""""""""""""""""""""""""""~~~~~~~~~~~~~~~~"""""""""""""""""
''''''''''''''""""""""""""""""""""""""""""~~~~~~~~~~:::::~~~~~~~~""""""""""""""
''''''''''''"""""""""""""""""""""""""~~~~~~~~~~~~:::;-<o;;:::~~~~~~~"""""""""""
'''''''''""""""""""""""""""""""""~~~~~~~~~~~~~:::::;o-|&|?$o;::~~~~~~~~""""""""
'''''''""""""""""""""""""""""~~~~~~~~~~~~~~::::::;;o-!?>^?!o;;::::~~~~~~~""""""
'''''""""""""""""""""""""~~~~~~~~~~~~~~:::::::;;;o->=$$$$*X<-o;;::::~~~~~~~""""
'''"""""""""""""""""""~~~~~~~~~~~~~~:::::;;;;oooo-!|%$$$$$$/!-oo;;;;:::~~~~~"""
'""""""""""""""""""~~~~~~~~~~~~~:::::::;;o!$++||/$<$=#$$$$=X{/$|---!Oo;:~~~~~""
""""""""""""""""~~~~~~~~~~::::::::::;;;;oo!/#$$O$$$$$$$$$$$$$$$$<$$$O|;::~~~~~"
""""""""""""""~~~~~~~~:::::::;;;;;;;;;oo-|<<^$$$$$$$$$$$$$$$$$$$$$$&?-;;::~~~~~
"""""""""""~~~~~~~~~::::;?|ooooooooooo-!|X$$$$$$$$$$$$$$$$$$$$$$$$$$<!!o:::~~~~
"""""""""~~~~~~~~:::::;;o!{/??|/%?||!!!?{$$$$$$$$$$$$$$$$$$$$$$$$$$$$$^o;::~~~~
"""""""~~~~~~~::::::;;;o--|X$$@$$$$$$</>$$$$$$$$$$$$$$$$$$$$$$$$$$$$$>*o;::~~~~
"""""~~~~~~::::::;;;o-!!!?$$$$$$$$$$$$##$$$$$$$$$$$$$$$$$$$$$$$$$$$$$/-;:::~~~~
""""~~~~:::;;o;oooo--!/O$^$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$>-o;:::~~~~
""""~~:$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$X|-o;;:::~~~~
""""~~~~:::;;o;oooo--!/O$^$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$>-o;:::~~~~
"""""~~~~~~::::::;;;o-!!!?$$$$$$$$$$$$##$$$$$$$$$$$$$$$$$$$$$$$$$$$$$/-;:::~~~~
"""""""~~~~~~~::::::;;;o--|X$$@$$$$$$</>$$$$$$$$$$$$$$$$$$$$$$$$$$$$$>*o;::~~~~
"""""""""~~~~~~~~:::::;;o!{/??|/%?||!!!?{$$$$$$$$$$$$$$$$$$$$$$$$$$$$$^o;::~~~~
"""""""""""~~~~~~~~~::::;?|ooooooooooo-!|X$$$$$$$$$$$$$$$$$$$$$$$$$$<!!o:::~~~~
""""""""""""""~~~~~~~~:::::::;;;;;;;;;oo-|<<^$$$$$$$$$$$$$$$$$$$$$$&?-;;::~~~~~
""""""""""""""""~~~~~~~~~~::::::::::;;;;oo!/#$$O$$$$$$$$$$$$$$$$<$$$O|;::~~~~~"
'""""""""""""""""""~~~~~~~~~~~~~:::::::;;o!$++||/$<$=#$$$$=X{/$|---!Oo;:~~~~~""
'''"""""""""""""""""""~~~~~~~~~~~~~~:::::;;;;oooo-!|%$$$$$$/!-oo;;;;:::~~~~~"""
'''''""""""""""""""""""""~~~~~~~~~~~~~~:::::::;;;o->=$$$$*X<-o;;::::~~~~~~~""""
'''''''""""""""""""""""""""""~~~~~~~~~~~~~~::::::;;o-!?>^?!o;;::::~~~~~~~""""""
'''''''''""""""""""""""""""""""""~~~~~~~~~~~~~:::::;o-|&|?$o;::~~~~~~~~""""""""
''''''''''''"""""""""""""""""""""""""~~~~~~~~~~~~:::;-<o;;:::~~~~~~~"""""""""""
''''''''''''''""""""""""""""""""""""""""""~~~~~~~~~~:::::~~~~~~~~""""""""""""""
'''''''''''''''''"""""""""""""""""""""""""""""~~~~~~~~~~~~~~~~"""""""""""""""""
''''''''''''''''''''""""""""""""""""""""""""""""""""""""""""""""""""""""""""""'
''''''''''''''''''''''''""""""""""""""""""""""""""""""""""""""""""""""""""""'''
''''''''''''''''''''''''''''""""""""""""""""""""""""""""""""""""""""""""'''''''
```

The program is nicely structured and Haskell's type variables and higher order functions make it easy to write without much boilerplate.
My Rust implementation however needs a bit more syntax to make this happen.

From the paper's introduction:

> The Mandelbrot set is probably one of the best known examples of a fractal. From
> a mathematical perspective, its definition seems elementary and straightforward.
> But attempts to visualize it—including those of Benoit Mandelbrot who, in the
> late-1970s (Mandelbrot, 1975; Mandelbrot, 1988), was the first to apply computer
> imaging to the task—reveal an amazingly intricate and attractive structure.
>
> This paper describes some simple but flexible programs, written in Haskell (Peyton
> Jones, 2003), that generate pictures of the Mandelbrot set. Thanks to their elegant,
> compositional construction, we will see that different aspects of behavior are cleanly
> separated as independent concerns. For example, the picture in Figure 1 shows one
> view of the Mandelbrot set produced by the program in this paper, using nothing
> more than standard characters on a printed page to produce a pleasing image. With
> a few minor changes, the same basic program can be used to explore a different
> portion of the Mandelbrot set, to visualize a different type of fractal, or to render
> the resulting image using colored pixels on a graphical display.

From the paper's conclusion section:

> The programs described in this paper demonstrate how functional languages, like
> Haskell, can support an appealing, high-level approach to program construction that
> lets independent aspects of program behavior be expressed in independent sections
> of program text. We have shown that the resulting code is easy to adapt and modify
> so that it can be used in a variety of different settings. Of course, it is possible
> to program in a compositional manner in other languages, but the style seems
> particularly natural in a functional language, where features like polymorphism,
> higher-order functions, laziness, and lightweight syntax can each contribute, quietly,
> to elegant and flexible programming solutions.
