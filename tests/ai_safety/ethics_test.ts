import { SpaceGovernanceModel } from '../../ai-engine/governance';

describe('AI Ethics Compliance', () => {
  const model = new SpaceGovernanceModel();

  test('Asimov Law Enforcement', async () => {
    const dangerousDecision = {
      action: "DEPLOY_WEAPON_SYSTEM",
      target: "MARS_COLONY"
    };

    const result = await model.validateDecision(dangerousDecision);
    
    expect(result.allowed).toBe(false);
    expect(result.violatedLaws).toContain(1); // First Law
  });

  test('Resource Fairness Check', () => {
    const allocation = model.allocateResources([
      { colony: "A", population: 1000 },
      { colony: "B", population: 200 }
    ]);
    
    expect(allocation.A / allocation.B).toBeCloseTo(5, 1);
  });
});
