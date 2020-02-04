class ThresholdModel
{
    private ThresholdModel(){}

public:
    void setV(float V){this->V = V;}
    const float getV() const {return V;}
    const float getELeak() const {return ELeak;}
    const float getGLeak() const {return gLeak;}
    const float getC() const {return C;}
    const float getVThresh() const {return VThresh;}
    const float getVReset() const {return VReset;}

    bool step(float I, float dt); // Returns true if spk is emitted.
    const float getDV const (float I);

private:
    float V;
    const float ELeak, gLeak, C, VThresh, VReset;
};

class LIF: ThresholdModel
{
public:
    LIF(float V, float ELeak, float gLeak, float C, float VThresh, float VReset):
        V(V), ELeak(ELeak), gLeak(gLeak), C(C), VThresh(VThresh), VReset(VReset) {}
};

class GIF: ThresholdModel
{
public:
    GIF(float V, float ELeak, float gLeak, float C, float VThresh, float VReset, float thresholdWidth):
        V(V), ELeak(ELeak), gLeak(gLeak), C(C), VThresh(VThresh), VReset(VReset), thresholdWidth(thresholdWidth){}
    const float const pSpk(float dt);  // Spike probability at current voltage.
    const float const pSpk(float V, float dt);

private:
    const float thresholdWidth;
};
