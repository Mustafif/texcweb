# TexCreate 3.2.0 Plans 

I haven't been working on TexCreate lately because the current 3.1 version works well without any significant issues with my daily writing. However, I have noticed a latency issue when finding updates, which causes me to use the `--ignore true` flag frequently. The Github release strategy could be more scalable, and I could not create a working database model that was simple for me to maintain.

While researching ways to centrally distribute my books, I came across MongoDB, and I was surprised at how easy it was to perform CRUD operations on a __free__ database cluster. I am considering making improvements to notify users if they are waiting for new updates, allowing them to try reconnecting or ignore it after a specific timeout.

My next plan is to release TexCreate templates v4, which will include four more templates, and I plan on migrating my current release system to MongoDB. This will require a refactor; I don't expect this release to be ready for a few months. However, I am excited about the scalability and performance improvements this change promises to bring.