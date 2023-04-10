**Hello World!** I am happy to introduce the first issue of the TexCreate blog posts which will be used to 
serve as a way to get the latest news of the TexCreate project. I plan on giving monthly updated as seen in 
other projects which will be a great way to show the different changes for the different TexCreate projects. 

As I am happy to announce the release of the third major version of TexCreate, it can be difficult to rewrite a whole project 
while still trying to keep the core concepts so those migrating won't be too lost. Although I will admit that this version is a 
lot different compared to the difference between version 1 to 2, hell that migration literally had a `migrate` command. 

This version introduced the largest workspace we have seen in TexCreate, to get a better idea, here's a quick summary 
of what each repo does taken from the [texc_v3_sources](https://github.com/MKProj/texc_v3_sources) repo. 

- [texcgen](https://github.com/MKProj/texcgen)
    - A Template Generator for TexCreate.
- [texcreate](https://github.com/MKProj/texcreate)
    - A LaTeX Project Creator.
- [texcweb](https://github.com/MKProj/texcweb)
    - Manages the TexCreate web service
- [mkproj_texcgen](https://github.com/MKProj/mkproj_texcgen)
    - A Template Generator customized for the MKProject first party templates.
- [texcreate_repo](https://github.com/MKProj/texcreate_repo)
    - Provides the `Repo` type for TexCreate, a way to manage template releases.
- [texcore](https://github.com/MKProj/texcore)
    - Create LaTeX using native Rust types (provides TexCreate `Template` type).
- [texcreate_v3_compiler_conf](https://github.com/MKProj/texcreate_v3_compiler_conf)
    - Provides the `Compiler` type for TexCreate and related projects.
- [texc_v3_web](https://github.com/MKProj/texc_v3_web)
    - Provides a local web application to build TexCreate projects.

