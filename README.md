# gig
A command line tool to rapily scaffold projects using the power of YAML

## Usage

```bash
$ gig <SCAFFOLD.YML FILE> [Project name]

        --git-init, -g, -i ~   Initalize git repository
```

### The YAML file structure follows the following convention:
```
folder:
        file.ext: "The contents of the file"
        README.md: "
        # Strings can be more than one line
        ## So many lines!
        "
```


And there can be multiple root nodes
```
folder:
        file.ext: "The contents of the file"
        README.md: "
        # Strings can be more than one line
        ## So many lines!
        "
folder:
        file.ext: "The contents of the file"
        README.md: "
        # Strings can be more than one line
        ## So many lines!
        "
```

## Important note(s)
The project name param also adheres to directory conventions
and `gig ./egg.yml egg` creates executes the scaffold in the egg directory
so `gig ./egg.yml .` creates executes the scaffold in the current directory
so if you use the --git-init flag it will initilize the repo in that place.
