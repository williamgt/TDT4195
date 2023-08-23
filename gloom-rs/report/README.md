To use this skeleton, install [pandoc](https://pandoc.org/installing.html).
We provide a working nix shell for those familiar with nix.

To compile `myfile.md` to PDF on linux, run:

    make myfile.pdf

To compile and view our provided example,  `template.md`, run:

    make template.pdf

On Windows, use pandoc directly:

    pandoc template.md -o template.pdf
