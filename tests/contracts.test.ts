import { describe, it, expect } from "vitest";
import {
  canExport,
  profiles,
  validateBindings,
  OfflineAdvertisementAdapter,
} from "../src/contracts";
import { catalogs, message } from "../src/i18n";
describe("honest capability and compatibility contracts", () => {
  it("never certifies an unimplemented exporter", () => {
    for (const p of profiles)
      expect(
        canExport(
          p,
          ["A", "B", "C", "D", "E", "F"].map((id) => ({
            id,
            status: "pass",
            evidence: ["synthetic"],
          })),
        ),
      ).toBe(false);
    expect(profiles.filter((p) => p.id.startsWith("vbridger-"))).toHaveLength(
      2,
    );
  });
  it("detects missing, conflicting and invalid expression mappings", () => {
    expect(
      validateBindings(
        [
          {
            semantic: "jaw.open",
            target: "CreatorJaw",
            min: 0,
            neutral: 0,
            max: 1,
          },
        ],
        ["jaw.open"],
      ),
    ).toEqual([]);
    expect(
      validateBindings(
        [{ semantic: "jaw.open", target: "X", min: 1, neutral: 0, max: 0 }],
        ["eye.left.blink"],
      ),
    ).toEqual(["invalid_range", "missing:eye.left.blink"]);
    expect(
      validateBindings(
        [
          { semantic: "a", target: "X", min: 0, neutral: 0, max: 1 },
          { semantic: "b", target: "X", min: 0, neutral: 0, max: 1 },
        ],
        [],
      ),
    ).toContain("duplicate_binding");
  });
  it("does not fetch advertisements in offline mode", async () =>
    expect(await new OfflineAdvertisementAdapter().banner("ja")).toBeNull());
});
describe("localization", () => {
  it("has identical keys and useful fallback for all locales", () => {
    for (const locale of ["ko", "ja", "en"] as const) {
      expect(Object.keys(catalogs[locale]).sort()).toEqual(
        Object.keys(catalogs.en).sort(),
      );
      expect(message(locale, "unknown_error")).toBe(catalogs[locale].error);
      for (const value of Object.values(catalogs[locale]))
        expect(value.length).toBeGreaterThan(0);
    }
  });
});
