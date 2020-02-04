#include "models.h"
#include <math.h>

bool LIF::step(float I, float dt)
{
    if(V >= VThresh)
    {
        V = VReset;
        return 1;
    }
    else
    {
        V = V + getDV(I) * dt;
        return 0;
    }
}

const float LIF::getDV(float I) const
{
    float lhs = (-gLeak * (V - ELeak) + I)/C;
    return lhs;
}

bool GIF::step(float I, float dt)
{
    float randUnif = float(rand()) / float(RAND_MAX);
    float spkProb = pSpk();
    if(float spkProb >= randUnif)
    {
        V = VReset;
        return 1;
    }
    else
    {
        V = V + getDV(I) * dt;
        return 0;
    }
}

const float GIF::getDV(float I) const
{
    float lhs = (-gLeak * (V - ELeak) + I)/C;
    return lhs;
}

const float GIF::pSpk(float V, float dt) const
{
    float distAboveThreshold = (V - Vthresh) / thresholdWidth;
    float firingRate = 0.001 * exp(distAboveThreshold);  // Rate in mHz.
    float pNoSpk =  exp(-firingRate * dt);
    return 1 - pNoSpk;
}

const float GIF::pSpk(float dt) const
{
    return pSpk(V, dt);
}
