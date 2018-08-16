## Запуск

### Компилирование

    mmc hello-world.m

### Запуск

    ./hello

### Addistional info

Don't forget to add /usr/local/mercury-rotd-2018-08-15/bin to your PATH,
-- /usr/local/mercury-rotd-2018-08-15/share/man to your MANPATH,
-- and /usr/local/mercury-rotd-2018-08-15/share/info to your INFOPATH,
-- to copy deep_profiler/mdprof_cgi to /usr/lib/cgi-bin,
-- and to add the following lines to the `.emacs' file
-- in your home directory:
	(add-to-list 'load-path 
		"/usr/local/mercury-rotd-2018-08-15/lib/mercury/elisp")
	(autoload 'mdb "gud" "Invoke the Mercury debugger" t)

