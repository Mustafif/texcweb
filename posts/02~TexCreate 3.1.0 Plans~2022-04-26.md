# TexCreate 3.1.0 Plans 

This is going to be the first update to TexCreate v3, and it won't be big, but it is important. I plan for 
TexCreate 3.1.0 to introduce a better update notification system, as well as updating to TexCore `0.7.1` and 
TexCreate_Repo `0.2.1`. This will bring changes to avoid breakage when updating to newer template repos in the 
chance that the latest one uses a TexCore version incompatible with yours. 

Inside the `Repo` type we have included a minimum TexCreate type as seen below: 
```{rust}
/// A Metadata file used to maintain the TexCreate Template Releases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo{
    /// Current version of the repo
    current_vers: u64,
    /// Minimum required TexCreate version
    texcreate: Version, 
    /// Number of templates
    num: u64,
    /// Contains a HashMap of the `<Name, Description>`
    info: HashMap<String, String>
}
```

With this new field `texcreate`, we will be able to compare your current TexCreate version and the required 
to see if there is any issue. I hope in the future, the difference between TexCore `0.x` and `0.y` won't cause issues, 
but at the current pace we are seeing these issues every new minor version. 

You will see commits to the various TexCreate repos in anticipation to these latest changes, and I hope that I could 
change my release schedule of this project to deliver a minor update around every 2-3 months. 

Thanks for using TexCreate,   
Mustafif Khan 