import { useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { RotateCcw } from "lucide-react";
import { useTranslation } from "react-i18next";

import type { GameStateData, PlayerData } from "../store/gameStore";
import type { PitchSlotRow } from "./SquadTab.helpers";
import {
  describePitchSlot,
  resolveTacticalRoles,
  tacticalRoleFitClassName,
  tacticalRoleFitScore,
  tacticalRolesForPitchPosition,
  translateTacticalRoleDescription,
  translateTacticalRoleLabel,
} from "./tacticalRoles";
import { Card, CardBody, CardHeader } from "./ui";

interface TacticsRoleAssignmentsPanelProps {
  formation: string;
  onGameUpdate: (gameState: GameStateData) => void;
  pitchSlotRows: PitchSlotRow[];
  tacticalRoles?: string[];
}

export default function TacticsRoleAssignmentsPanel({
  formation,
  onGameUpdate,
  pitchSlotRows,
  tacticalRoles,
}: TacticsRoleAssignmentsPanelProps) {
  const { t } = useTranslation();

  const resolvedRoles = useMemo(
    () => resolveTacticalRoles(formation, pitchSlotRows, tacticalRoles),
    [formation, pitchSlotRows, tacticalRoles],
  );

  async function persistRoles(nextRoles: string[]): Promise<void> {
    try {
      const updated = await invoke<GameStateData>("set_tactical_roles", {
        tacticalRoles: nextRoles,
      });
      onGameUpdate(updated);
    } catch (error) {
      console.error("Failed to set tactical roles:", error);
    }
  }

  async function handleRoleChange(
    slotIndex: number,
    nextRole: string,
  ): Promise<void> {
    const nextRoles = [...resolvedRoles];
    nextRoles[slotIndex] = nextRole as (typeof resolvedRoles)[number];
    await persistRoles(nextRoles);
  }

  async function handleResetDefaults(): Promise<void> {
    await persistRoles(resolveTacticalRoles(formation, pitchSlotRows));
  }

  return (
    <Card>
      <CardHeader>{t("tactics.tacticalRolesTitle", "Tactical Roles")}</CardHeader>
      <CardBody>
        <div className="mb-4 flex items-center justify-between gap-3 rounded-xl border border-gray-200 bg-gray-50 px-4 py-3 dark:border-navy-600 dark:bg-navy-800/70">
          <div>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              {t(
                "tactics.tacticalRolesHint",
                "Assign a job to each slot in the current shape. Roles change how the engine uses that player in buildup, defending, and chance creation.",
              )}
            </p>
            <p className="mt-1 text-xs uppercase tracking-wider text-gray-500 dark:text-gray-400">
              {t("tactics.currentShape", "Current shape")}: {formation}
            </p>
          </div>
          <button
            type="button"
            onClick={() => {
              void handleResetDefaults();
            }}
            className="shrink-0 rounded-lg bg-gray-900 px-3 py-2 text-xs font-heading font-bold uppercase tracking-wider text-white transition-colors hover:bg-gray-700 dark:bg-navy-600 dark:hover:bg-navy-500"
          >
            <span className="flex items-center gap-2">
              <RotateCcw className="h-3.5 w-3.5" />
              {t("tactics.resetRoleDefaults", "Reset defaults")}
            </span>
          </button>
        </div>

        <div className="space-y-4">
          {pitchSlotRows.map((row) => (
            <section key={row.label}>
              <div className="mb-2 text-xs font-heading font-bold uppercase tracking-widest text-gray-500 dark:text-gray-400">
                {t(`tactics.pitchRows.${row.label}`, row.label)}
              </div>
              <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
                {row.slots.map((slot) => {
                  const player = slot.player as PlayerData | null;
                  const role = resolvedRoles[slot.index];
                  const fitScore = player ? tacticalRoleFitScore(player, role) : null;
                  const fitText = fitScore === null ? "-" : fitScore.toString();
                  return (
                    <div
                      key={`${row.label}-${slot.index}`}
                      className="rounded-xl border border-gray-200 bg-white p-4 dark:border-navy-600 dark:bg-navy-800/70"
                    >
                      <div className="mb-3 flex items-start justify-between gap-3">
                        <div>
                          <div className="text-[10px] font-heading font-bold uppercase tracking-widest text-gray-500 dark:text-gray-400">
                            {describePitchSlot(t, slot.position)}
                          </div>
                          <div className="mt-1 text-sm font-semibold text-gray-900 dark:text-gray-100">
                            {player?.full_name ?? t("common.emptySlot", "Empty slot")}
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="text-[10px] font-heading font-bold uppercase tracking-widest text-gray-500 dark:text-gray-400">
                            {t("tactics.roleFit", "Role fit")}
                          </div>
                          <div
                            className={`text-lg font-heading font-bold tabular-nums ${
                              fitScore === null
                                ? "text-gray-500 dark:text-gray-400"
                                : tacticalRoleFitClassName(fitScore)
                            }`}
                          >
                            {fitText}
                          </div>
                        </div>
                      </div>

                      <label className="block">
                        <span className="mb-1 block text-[10px] font-heading font-bold uppercase tracking-widest text-gray-500 dark:text-gray-400">
                          {t("tactics.assignedRole", "Assigned role")}
                        </span>
                        <select
                          aria-label={`${player?.full_name ?? slot.position} role`}
                          value={role}
                          onChange={(event) => {
                            void handleRoleChange(slot.index, event.target.value);
                          }}
                          className="w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 focus:border-primary-500 focus:outline-none dark:border-navy-600 dark:bg-navy-900 dark:text-gray-100"
                        >
                          {tacticalRolesForPitchPosition(slot.position).map((option) => (
                            <option key={option} value={option}>
                              {translateTacticalRoleLabel(t, option)}
                            </option>
                          ))}
                        </select>
                      </label>

                      <p className="mt-3 text-sm leading-relaxed text-gray-600 dark:text-gray-300">
                        {translateTacticalRoleDescription(t, role)}
                      </p>
                    </div>
                  );
                })}
              </div>
            </section>
          ))}
        </div>
      </CardBody>
    </Card>
  );
}
