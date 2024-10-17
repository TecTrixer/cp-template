# Abbreviation for running just
abbr j just

# Folder where this repo is:
set CP_REPO (realpath (dirname (status filename)))

# Include cpp config
# source {$CP_REPO}/cpp/config.fish


# This function adds another subtask to an already established contest
function cp_add_task
    if test (count $argv) -lt 1
        echo "You need to specify the task name, e.g. 'addcp b' will add 'b.rs'"
    else
        set i $argv[1]
        echo "[[bin]]" >>Cargo.toml
        echo "name=\"$i\"" >>Cargo.toml
        echo "path=\"$i.rs\"" >>Cargo.toml
        echo "" >>Cargo.toml
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
            cp_add_task (printf "\\$(printf '%03o' $val)")
        end
    end
end

# This function creates a new contest and optionally adds files for the subtasks
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
        # also automatically start competitive companion server in the future
        comp-helper &
        bg
    end
end

# this function evaluates a test and compares the output using delta
function cp_eval_test
    if test (count $argv) -eq 2
        echo "Testing task "(echo $argv[2] | string upper)
        echo ""
        for in_file in ./{$argv[2]}*.in
            echo (echo $in_file | string replace "./" "")":"
            set out_file (echo $in_file | string replace ".in" ".out")
            begin
                set -l IFS
                set res (bash -c "diff -U1000 -u <(cat $in_file; echo '---'; cat $in_file | $argv[1] | awk '{\$1=\$1};1') <(cat $in_file; echo '---'; cat $out_file)")
            end
            if test $status -eq 0
                echo "~~~~~~"
                bash -c "pr -m -t <(cat $in_file; echo '---'; cat $in_file | $argv[1] | awk '{\$1=\$1};1') <(cat $in_file; echo '---'; cat $out_file)"
            else
                echo -n "~~~~~~"
                echo $res | delta
            end
            echo ""
        end
    else
        echo "You need to specify the program binary path and the name of the test"
    end
end
