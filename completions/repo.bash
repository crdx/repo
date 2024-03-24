#!/bin/bash
# shellcheck disable=SC2207

function __repo {
    local cur
    cur="${COMP_WORDS[COMP_CWORD]}"

    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=($(compgen -W '--dirty --unpushed --all --absolute --verbose --interactive --ignore-errors ls run' -- "$cur"))
    else
        case ${COMP_WORDS[1]} in
            ls) __repo_ls ;;
            run) __repo_run ;;
        esac
    fi
}

function __repo_ls {
    local cur
    cur="${COMP_WORDS[COMP_CWORD]}"

    if [ "$COMP_CWORD" -ge 2 ]; then
        COMPREPLY=($(compgen -W '--dirty --unpushed --all --absolute --verbose --interactive --ignore-errors' -- "$cur"))
    fi
}

function __repo_run {
    local cur
    cur="${COMP_WORDS[COMP_CWORD]}"

    if [ "$COMP_CWORD" -eq 2 ]; then
        COMPREPLY=($(compgen -W '--dirty --unpushed --all --absolute --verbose --interactive --ignore-errors --' -- "$cur"))
    else
        case ${COMP_WORDS[2]} in
            --) __repo_run_-- ;;
        esac
    fi
}

function __repo_run_-- {
    local cur
    cur="${COMP_WORDS[COMP_CWORD]}"

    if [ "$COMP_CWORD" -ge 3 ]; then
        COMPREPLY=($(compgen -W '--dirty --unpushed --all --absolute --verbose --interactive --ignore-errors' -- "$cur"))
    fi
}

complete -o bashdefault -o default -o filenames -F __repo repo
