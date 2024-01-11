# Folder where this repo is:
set CP_REPO "~/cp/cp-template"

# Abbreviation for running just
abbr j just

# This function adds another subtask to an already established contest
function cp_add_task
    if test (count $argv) -lt 1
        echo "You need to specify the task name, e.g. 'addcp b' will add 'b.rs'"
    else
        set i $argv[1]
        echo "[[bin]]" >> Cargo.toml
        echo "name=\"$i\"" >> Cargo.toml
        echo "path=\"$i.rs\"" >> Cargo.toml
        echo "" >> Cargo.toml
        cp {$CP_REPO}/temp.rs ./$i.rs
    end
end

# This function adds files for each subtask of the problem, e.g. from a.rs until h.rs
function cp_add_contest_files
    if test (count $argv) -lt 2
        echo "You need to specify the start and end of the files you want to add"
    else
        set start_val (printf "%d" "'$argv[1]'")
        set end_val (printf "%d" "'$argv[2]'")
        for val in (seq $start_val $end_val)
            cp_add_task (printf "\\$(printf '#03o')")
        end
    end
end

function cp_new
    if test (count $argv) -ge 1
        mkdir $argv[1]
        cd $argv[1]
        cp {$CP_REPO}/{Cargo.toml,.ignore,justfile,template.rs} .
    else
        echo "Usage: [DIRNAME] [FROM] [TO], where [FROM] and [TO] are optional"    
    end
    if test (count $argv) -eq 3
        cp_add_contest_files $argv[2] $argv[3]
        # TODO: also automatically start competitive companion server in the future
    end
end
