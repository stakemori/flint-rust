#include "flint.h"
#include "fmpz.h"
#include "fmpz_factor.h"
#include "fmpz_mat.h"
#include "fmpz_poly.h"
#include "fmpz_vec.h"

void wrapped_fmpz_clear(fmpz_t f) { fmpz_clear(f); }

void wrapped_fmpz_init(fmpz_t f) { fmpz_init(f); }

void wrapped_fmpz_init_set(fmpz_t f, const fmpz_t g) { fmpz_init_set(f, g); }

void wrapped_fmpz_init_set_si(fmpz_t f, slong g) { fmpz_init_set_si(f, g); }

void wrapped_fmpz_init_set_ui(fmpz_t f, ulong g) { fmpz_init_set_ui(f, g); }

void wrapped_fmpz_set_si(fmpz_t f, slong val) { fmpz_set_si(f, val); }

void wrapped_fmpz_set_ui(fmpz_t f, ulong val) { fmpz_set_ui(f, val); }

void wrapped_fmpz_neg_ui(fmpz_t f, ulong val) { fmpz_neg_ui(f, val); }

void wrapped_fmpz_set_uiui(fmpz_t f, mp_limb_t hi, mp_limb_t lo)
{
  fmpz_set_uiui(f, hi, lo);
}

void wrapped_fmpz_neg_uiui(fmpz_t f, mp_limb_t hi, mp_limb_t lo)
{
  fmpz_neg_uiui(f, hi, lo);
}

void wrapped_fmpz_zero(fmpz_t f) { fmpz_zero(f); }

void wrapped_fmpz_one(fmpz_t f) { fmpz_one(f); }

int wrapped_fmpz_is_zero(const fmpz_t f) { return fmpz_is_zero(f); }

int wrapped_fmpz_is_one(const fmpz_t f) { return fmpz_is_one(f); }

void wrapped_fmpz_swap(fmpz_t f, fmpz_t g) { fmpz_swap(f, g); }

int wrapped_fmpz_is_even(const fmpz_t f) { return fmpz_is_even(f); }

int wrapped_fmpz_is_odd(const fmpz_t f) { return fmpz_is_odd(f); }

void wrapped_fmpz_neg(fmpz_t f1, const fmpz_t f2) { fmpz_neg(f1, f2); }

void wrapped_fmpz_negmod(fmpz_t r, const fmpz_t a, const fmpz_t mod)
{
  fmpz_negmod(r, a, mod);
}

void wrapped_fmpz_mul2_uiui(fmpz_t f, const fmpz_t g, ulong h1, ulong h2)
{
  fmpz_mul2_uiui(f, g, h1, h2);
}

void wrapped_fmpz_divexact2_uiui(fmpz_t f, const fmpz_t g, ulong h1, ulong h2)
{
  fmpz_divexact2_uiui(f, g, h1, h2);
}

void wrapped_flint_randinit(flint_rand_t state) { flint_randinit(state); }

void wrapped_flint_randclear(flint_rand_t state) { flint_randclear(state); }

void wrapped_fmpz_set_ui_smod(fmpz_t f, mp_limb_t x, mp_limb_t m)
{
  fmpz_set_ui_smod(f, x, m);
}

int wrapped_fmpz_is_pm1(const fmpz_t f) { return fmpz_is_pm1(f); }
