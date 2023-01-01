# USDZ Rust Parser

This project is meant to be a USDZ parser for the web projects written in WebAssembly.

# What is USDZ

USDZ is a file format for 3D models. It is a zip file containing a USD file and resources (images, etc). The USD file is a text file that describes the 3D model. The resources are the textures and other files that the USD file references. Example:

```usd
#usda 1.0

def Xform "hello"
{
    def Sphere "world"
    {
        
    }
}
```

# How to create a USDZ file

1. Create a directory of USD files and resources.
2. Zip the directory `zip -0 -R my_file.zip usdz_directory`
3. Rename `my_file.zip` to `my_file.usdz`
