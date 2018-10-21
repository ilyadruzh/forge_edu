/*
** Automatically generated from `hello.m'
** by the Mercury compiler,
** version 14.01.1, configured for x86_64-unknown-linux-gnu.
** Do not edit.
**
** The autoconfigured grade settings governing
** the generation of this C file were
**
** TAG_BITS=3
** UNBOXED_FLOAT=yes
** PREGENERATED_DIST=no
** HIGHLEVEL_CODE=no
**
** END_OF_C_GRADE_INFO
*/

/*
INIT mercury__hello__init
ENDINIT
*/

#define MR_ALLOW_RESET
#include "mercury_imp.h"
#line 526 "/usr/local/mercury-14.01.1/lib/mercury/ints/io.int"
#include "io.mh"

#line 29 "hello.c"
#line 536 "/usr/local/mercury-14.01.1/lib/mercury/ints/io.int"
#include "time.mh"

#line 33 "hello.c"
#line 537 "/usr/local/mercury-14.01.1/lib/mercury/ints/io.int"
#include "string.mh"

#line 37 "hello.c"
#line 31 "/usr/local/mercury-14.01.1/lib/mercury/ints/bitmap.int2"
#include "bitmap.mh"

#line 41 "hello.c"
#line 42 "hello.c"
#include "hello.mh"

#line 45 "hello.c"
#line 46 "hello.c"
#ifndef HELLO_DECL_GUARD
#define HELLO_DECL_GUARD

#line 50 "hello.c"
#line 51 "hello.c"

#endif
#line 54 "hello.c"

#ifdef _MSC_VER
#define MR_STATIC_LINKAGE extern
#else
#define MR_STATIC_LINKAGE static
#endif
MR_def_extern_entry(main_2_0)



MR_decl_entry(io__write_string_3_0);

MR_BEGIN_MODULE(hello_module0)
	MR_init_entry1(main_2_0);
	MR_INIT_PROC_LAYOUT_ADDR(mercury__main_2_0);
MR_BEGIN_CODE

/*-------------------------------------------------------------------------*/
/* code for 'main'/2 mode 0 */
#ifdef MR_maybe_local_thread_engine_base
	#undef MR_maybe_local_thread_engine_base
	#define MR_maybe_local_thread_engine_base MR_local_thread_engine_base
#endif
MR_define_entry(mercury__main_2_0);
	MR_MAYBE_INIT_LOCAL_THREAD_ENGINE_BASE
	MR_r1 = (MR_Word) MR_string_const("Hello, World!\n", 14);
	MR_np_tailcall_ent(io__write_string_3_0);
#ifdef MR_maybe_local_thread_engine_base
	#undef MR_maybe_local_thread_engine_base
	#define MR_maybe_local_thread_engine_base MR_thread_engine_base
#endif
MR_END_MODULE

static void mercury__hello_maybe_bunch_0(void)
{
	hello_module0();
}

/* suppress gcc -Wmissing-decls warnings */
void mercury__hello__init(void);
void mercury__hello__init_type_tables(void);
void mercury__hello__init_debugger(void);
#ifdef MR_DEEP_PROFILING
void mercury__hello__write_out_proc_statics(FILE *deep_fp, FILE *procrep_fp);
#endif
#ifdef MR_RECORD_TERM_SIZES
void mercury__hello__init_complexity_procs(void);
#endif
#ifdef MR_THREADSCOPE
void mercury__hello__init_threadscope_string_table(void);
#endif

void mercury__hello__init(void)
{
	static MR_bool done = MR_FALSE;
	if (done) {
		return;
	}
	done = MR_TRUE;
	mercury__hello_maybe_bunch_0();
	mercury__hello__init_debugger();
}

void mercury__hello__init_type_tables(void)
{
	static MR_bool done = MR_FALSE;
	if (done) {
		return;
	}
	done = MR_TRUE;
}


void mercury__hello__init_debugger(void)
{
	static MR_bool done = MR_FALSE;
	if (done) {
		return;
	}
	done = MR_TRUE;
}

#ifdef MR_DEEP_PROFILING

void mercury__hello__write_out_proc_statics(FILE *deep_fp, FILE *procrep_fp)
{
	MR_write_out_module_proc_reps_start(procrep_fp, &mercury_data__module_layout__hello);
	MR_write_out_module_proc_reps_end(procrep_fp);
}

#endif

#ifdef MR_RECORD_TERM_SIZES

void mercury__hello__init_complexity_procs(void)
{
}

#endif

#ifdef MR_THREADSCOPE

void mercury__hello__init_threadscope_string_table(void)
{
}

#endif

/* ensure everything is compiled with the same grade */
static const void *const MR_grade = &MR_GRADE_VAR;
