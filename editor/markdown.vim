function! InsertCitation()
  let tempfile=tempname()
  execute '! clarence searchfzf -m > ' . shellescape(tempfile)
  execute 'read' . tempfile
  call delete(tempfile)
endfunction

inoreabbrev <buffer> [@ [<CR>]<Esc>k:call InsertCitation()<CR><CR>kJJf]a
inoreabbrev <buffer> @ <CR>]<Esc>k:call InsertCitation()<CR><CR>kJJcf]
