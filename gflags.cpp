#include "gflags.h"

using namespace GFLAGS_NAMESPACE;

std::string from_slice(str_slice s)
{
  return std::string(s.base, s.base + s.len);
}

FlagRegisterer *flag_registerer(str_slice name, str_slice type, str_slice help,
                                str_slice filename, void *current_storage,
                                void *defvalue_storage)
{
  auto s_name = from_slice(name);
  auto s_type = from_slice(type);
  auto s_help = from_slice(help);
  auto s_filename = from_slice(filename);

  return new FlagRegisterer(s_name.c_str(),
                            s_help.c_str(), s_filename.c_str(),
                            current_storage, defvalue_storage);
}

void free_flag_registerer(FlagRegisterer *flagreg) { delete flagreg; }

bool get_commandline_option(str_slice name, void (*set)(void *ctxt, str_slice value), void *ctxt)
{
  std::string ret;
  auto s_name = from_slice(name);

  bool found = GetCommandLineOption(s_name.c_str(), &ret);
  if (found)
  {
    str_slice s = {ret.data(), ret.length()};
    (*set)(ctxt, s);
  }

  return found;
}