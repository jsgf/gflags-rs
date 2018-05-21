#ifndef GFLAGS_H
#define GFLAGS_H

#pragma once

#include <gflags/gflags.h>

struct str_slice {
  const char *base;
  size_t len;
};

class GFLAGS_NAMESPACE::FlagRegisterer *
flag_registerer(str_slice name, str_slice type, str_slice help,
                str_slice filename, void *current_storage,
                void *defvalue_storage);

void free_flag_registerer(GFLAGS_NAMESPACE::FlagRegisterer *);

bool get_commandline_option(str_slice name, void (*set)(void *ctxt, str_slice value), void *ctxt);

#endif // GFLAGS_H