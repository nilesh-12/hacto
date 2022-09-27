
set number
set relativenumber
set autoindent
set smartindent
set nocul
set tabstop=4
set softtabstop=4
set shiftwidth=4
set clipboard=unnamedplus
set background=dark
set completeopt=noinsert,menuone,noselect
set inccommand=split
set hidden
set mouse=a
set splitbelow splitright
set ttimeoutlen=0
set wildmenu
set expandtab
set termguicolors
set scrolloff=8

filetype plugin indent on
syntax on

set t_Co=256
" true Italic support.
let &t_ZH="\e[3m"
let &t_ZR="\e[23m"

" FileBrowser
let g:netrw_banner=0
let g:netrw_liststyle=0
let g:netrw_altv=1
let g:netrw_winsize=25
let g:netrw_keepdir=0
let g:netrw_localcopydircmd='cp -r'

colo zellner
colo shine
set cul
colo morning
set nocul

" Creating file without opening buffer
function! CreateInPreview()
    let l:filename = input('please enter filename: ')
    execute 'silent !touch ' . b:netrw_curdir . '/' . l:filename
    redraw!
endfunction

" Netrw: create file using touch instead of opening a buffer
function! Netrw_mappings() 
    noremap <buffer>% :call CreateInPreview()<cr>
endfunction

augroup auto_commands
    autocmd filetype netrw call Netrw_mappings()
augroup END

" Tabs
nnoremap <S-Tab> gT
nnoremap <Tab> gt
nnoremap <silent> <S-t> :tabnew<CR>

runtime! ./init.vim






