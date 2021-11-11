// Generated by gir (https://github.com/gtk-rs/gir @ df67128a87f0)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 46f24acbe4c2)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) ADW_CENTERING_POLICY_LOOSE);
    PRINT_CONSTANT((gint) ADW_CENTERING_POLICY_STRICT);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_DEFAULT);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_FORCE_DARK);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_FORCE_LIGHT);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_PREFER_DARK);
    PRINT_CONSTANT((gint) ADW_COLOR_SCHEME_PREFER_LIGHT);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_ALWAYS);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_AUTO);
    PRINT_CONSTANT((gint) ADW_FLAP_FOLD_POLICY_NEVER);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_OVER);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_SLIDE);
    PRINT_CONSTANT((gint) ADW_FLAP_TRANSITION_TYPE_UNDER);
    PRINT_CONSTANT((gint) ADW_FOLD_THRESHOLD_POLICY_MINIMUM);
    PRINT_CONSTANT((gint) ADW_FOLD_THRESHOLD_POLICY_NATURAL);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_OVER);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_SLIDE);
    PRINT_CONSTANT((gint) ADW_LEAFLET_TRANSITION_TYPE_UNDER);
    PRINT_CONSTANT((gint) ADW_NAVIGATION_DIRECTION_BACK);
    PRINT_CONSTANT((gint) ADW_NAVIGATION_DIRECTION_FORWARD);
    PRINT_CONSTANT((gint) ADW_SQUEEZER_TRANSITION_TYPE_CROSSFADE);
    PRINT_CONSTANT((gint) ADW_SQUEEZER_TRANSITION_TYPE_NONE);
    PRINT_CONSTANT((gint) ADW_VIEW_SWITCHER_POLICY_NARROW);
    PRINT_CONSTANT((gint) ADW_VIEW_SWITCHER_POLICY_WIDE);
    return 0;
}
