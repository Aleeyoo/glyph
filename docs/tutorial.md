glyph Tutorial
============

glyph is a lightweight Emacs-compatible text editor.
This tutorial covers the basics.

Getting Started
---------------
Launch glyph:
  glyph file.txt

If no file is given, you start in the *scratch* buffer.

Basic Movement
--------------
  C-f     Forward one character
  C-b     Backward one character
  C-n     Next line
  C-p     Previous line
  C-a     Beginning of line
  C-e     End of line
  C-v     Scroll down (next screen)
  M-v     Scroll up (previous screen)

(M-x means press the META or ESC key, then x)

Editing
-------
Just type to insert text.

  C-d     Delete character at point
  DEL     Delete character before point
  C-k     Kill (cut) to end of line
  C-y     Yank (paste) the last kill

Files
-----
  C-x C-f  Find file (open)
  C-x C-s  Save buffer
  C-x C-c  Quit (prompts to save)

Help
----
  M-x describe-key       Show what a key does
  M-x describe-bindings  List all keybindings
  C-h ?                 Help menu

Configuration
-------------
glyph loads ~/.config/glyph/config.mg on startup.
This file uses the miniLisp extension language.

Example config.mg:
  ;; Set default tab width
  (setq-default indent-tabs-mode nil)
  (set-fill-column 72)
