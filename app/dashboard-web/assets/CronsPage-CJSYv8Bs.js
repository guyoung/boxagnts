import { T as defineStore, K as api, S as defineComponent, ab as useAppStore, a1 as onMounted, P as createElementBlock, M as createBaseVNode, R as createVNode, ai as withCtx, r as VIcon, e as VBtn, aa as unref, a8 as toDisplayString, O as createCommentVNode, z as VRow, N as createBlock, o as VDialog, a4 as ref, a2 as openBlock, Q as createTextVNode, F as Fragment, a5 as renderList, f as VCard, $ as normalizeClass, h as VCardItem, k as VCardTitle, i as VCardSubtitle, m as VChip, b as VAvatar, j as VCardText, g as VCardActions, al as withModifiers, C as VSpacer, D as VSwitch, n as VCol, H as VTextField, A as VSelect, I as VTextarea, q as VExpandTransition, p as VDivider, V as VAlert, x as VProgressCircular, t as VListItem, v as VListItemTitle, u as VListItemSubtitle, s as VList, L as computed, Z as nextTick, B as VSkeletonLoader, _ as _export_sfc } from "./main-BSD2YpbL.js";
import { u as useCrudOperations } from "./baseCrud-dWOHLWHe.js";
const useCronStore = defineStore("crons", () => {
  const crud = useCrudOperations(
    {
      fetchAll: () => api.getCrons(),
      create: (data) => api.createCron(data),
      update: (id, data) => api.updateCron(id, data),
      remove: (id) => api.deleteCron(id)
    },
    "crons"
  );
  async function fetchCronLogs(jobId) {
    return api.getCronLogs(jobId);
  }
  return {
    crons: crud.items,
    loading: crud.loading,
    fetchCrons: crud.fetch,
    addCron: crud.add,
    updateCron: crud.update,
    removeCron: crud.remove,
    fetchCronLogs
  };
});
const DAY_NAMES = {
  sun: 0,
  mon: 1,
  tue: 2,
  wed: 3,
  thu: 4,
  fri: 5,
  sat: 6
};
const MONTH_NAMES = {
  jan: 1,
  feb: 2,
  mar: 3,
  apr: 4,
  may: 5,
  jun: 6,
  jul: 7,
  aug: 8,
  sep: 9,
  oct: 10,
  nov: 11,
  dec: 12
};
function parseField(field, min, max, nameMap) {
  const result = /* @__PURE__ */ new Set();
  const parts = field.toLowerCase().split(",");
  for (const rawPart of parts) {
    let part = rawPart.trim();
    let step = 1;
    if (part.includes("/")) {
      const [range, stepStr] = part.split("/");
      part = range;
      step = parseInt(stepStr, 10);
      if (isNaN(step) || step <= 0) step = 1;
    }
    let start;
    let end;
    if (part === "*") {
      start = min;
      end = max;
    } else if (part.includes("-")) {
      const [s, e] = part.split("-");
      const sLower = s.trim().toLowerCase();
      const eLower = e.trim().toLowerCase();
      start = (nameMap == null ? void 0 : nameMap[sLower]) ?? parseInt(s, 10);
      end = (nameMap == null ? void 0 : nameMap[eLower]) ?? parseInt(e, 10);
    } else {
      const valLower = part.trim().toLowerCase();
      start = (nameMap == null ? void 0 : nameMap[valLower]) ?? parseInt(part, 10);
      end = start;
    }
    if (isNaN(start) || isNaN(end)) continue;
    for (let i = start; i <= end; i += step) {
      if (i >= min && i <= max) {
        result.add(i);
      }
    }
  }
  return result;
}
function getNextRunTimes(expr, count = 3) {
  const fields = expr.trim().split(/\s+/);
  if (fields.length !== 6) return [];
  const seconds = parseField(fields[0], 0, 59);
  const minutes = parseField(fields[1], 0, 59);
  const hours = parseField(fields[2], 0, 23);
  const days = parseField(fields[3], 1, 31);
  const months = parseField(fields[4], 1, 12, MONTH_NAMES);
  const dows = parseField(fields[5], 0, 6, DAY_NAMES);
  const results = [];
  const maxIter = 366 * 24 * 60;
  const now = /* @__PURE__ */ new Date();
  let current = new Date(now.getFullYear(), now.getMonth(), now.getDate(), now.getHours(), now.getMinutes() + 1, 0, 0);
  let iter = 0;
  while (results.length < count && iter < maxIter) {
    iter++;
    if (!months.has(current.getMonth() + 1)) {
      current.setMonth(current.getMonth() + 1, 1);
      current.setHours(0, 0, 0, 0);
      continue;
    }
    const dayValid = days.has(current.getDate());
    const dowValid = dows.has(current.getDay());
    const bothWildcard = fields[3].trim() === "*" && fields[5].trim() === "*";
    if (!bothWildcard && !(dayValid && dowValid) && !(fields[3].trim() !== "*" && dayValid || fields[5].trim() !== "*" && dowValid)) {
      current.setDate(current.getDate() + 1);
      current.setHours(0, 0, 0, 0);
      continue;
    }
    if (!dayValid && !dowValid) {
      current.setDate(current.getDate() + 1);
      current.setHours(0, 0, 0, 0);
      continue;
    }
    if (!hours.has(current.getHours())) {
      current.setHours(current.getHours() + 1, 0, 0, 0);
      continue;
    }
    if (!minutes.has(current.getMinutes())) {
      current.setMinutes(current.getMinutes() + 1, 0, 0);
      continue;
    }
    let secFound = false;
    for (let s = current.getSeconds(); s < 60; s++) {
      if (seconds.has(s)) {
        current.setSeconds(s);
        results.push(new Date(current));
        secFound = true;
        break;
      }
    }
    if (!secFound) {
      current.setMinutes(current.getMinutes() + 1, 0, 0);
      continue;
    }
    current.setSeconds(current.getSeconds() + 1);
    if (current.getSeconds() === 0) {
      current.setMinutes(current.getMinutes() + 1, 0, 0);
    }
  }
  return results;
}
function isValidCronExpr(expr) {
  const fields = expr.trim().split(/\s+/);
  return fields.length === 6;
}
const CRON_FIELD_DEFS = {
  second: [
    { title: "Every second", value: "*" },
    { title: "At :00", value: "0" },
    { title: "At :30", value: "30" },
    { title: "Every 5s", value: "*/5" },
    { title: "Every 15s", value: "*/15" },
    { title: "Every 30s", value: "*/30" }
  ],
  minute: [
    { title: "Every minute", value: "*" },
    { title: "At :00", value: "0" },
    { title: "At :15", value: "15" },
    { title: "At :30", value: "30" },
    { title: "At :45", value: "45" },
    { title: "Every 5 min", value: "*/5" },
    { title: "Every 15 min", value: "*/15" },
    { title: "Every 30 min", value: "*/30" }
  ],
  hour: [
    { title: "Every hour", value: "*" },
    { title: "Midnight (0)", value: "0" },
    { title: "6 AM", value: "6" },
    { title: "8 AM", value: "8" },
    { title: "9 AM", value: "9" },
    { title: "12 PM", value: "12" },
    { title: "6 PM", value: "18" },
    { title: "Every 2 hrs", value: "*/2" },
    { title: "Every 6 hrs", value: "*/6" }
  ],
  day: [
    { title: "Every day", value: "*" },
    { title: "1st", value: "1" },
    { title: "15th", value: "15" },
    { title: "Last (28)", value: "28" }
  ],
  month: [
    { title: "Every month", value: "*" },
    { title: "Jan", value: "1" },
    { title: "Feb", value: "2" },
    { title: "Mar", value: "3" },
    { title: "Apr", value: "4" },
    { title: "May", value: "5" },
    { title: "Jun", value: "6" },
    { title: "Jul", value: "7" },
    { title: "Aug", value: "8" },
    { title: "Sep", value: "9" },
    { title: "Oct", value: "10" },
    { title: "Nov", value: "11" },
    { title: "Dec", value: "12" }
  ],
  dow: [
    { title: "Every day", value: "*" },
    { title: "Mon–Fri", value: "Mon-Fri" },
    { title: "Sat–Sun", value: "Sun,Sat" },
    { title: "Mon", value: "Mon" },
    { title: "Tue", value: "Tue" },
    { title: "Wed", value: "Wed" },
    { title: "Thu", value: "Thu" },
    { title: "Fri", value: "Fri" },
    { title: "Sat", value: "Sat" },
    { title: "Sun", value: "Sun" }
  ]
};
function describeCron(expr) {
  const fields = expr.trim().split(/\s+/);
  if (fields.length !== 6) return "";
  const parts = [];
  const secLabel = lookUpLabel("second", fields[0]);
  const minLabel = lookUpLabel("minute", fields[1]);
  const hourLabel = lookUpLabel("hour", fields[2]);
  const dayLabel = lookUpLabel("day", fields[3]);
  const monthLabel = lookUpLabel("month", fields[4]);
  const dowLabel = lookUpLabel("dow", fields[5]);
  if (secLabel !== "-" && secLabel !== "Every second") {
    parts.push(`Seconds: ${secLabel}`);
  }
  const timeParts = [];
  if (minLabel !== "-" && minLabel !== "Every minute") timeParts.push(minLabel.toLowerCase());
  if (hourLabel !== "-" && hourLabel !== "Every hour") timeParts.push(`at ${hourLabel.toLowerCase()}`);
  if (timeParts.length > 0) parts.push(timeParts.join(" "));
  const dateParts = [];
  if (dowLabel !== "-" && dowLabel !== "Every day") dateParts.push(dowLabel);
  if (dayLabel !== "-" && dayLabel !== "Every day") dateParts.push(`day ${dayLabel}`);
  if (monthLabel !== "-" && monthLabel !== "Every month") dateParts.push(monthLabel);
  if (dateParts.length > 0) parts.push(`on ${dateParts.join(", ")}`);
  return parts.length > 0 ? parts.join(" — ") : "Every second of every day";
}
function lookUpLabel(field, value) {
  const opts = CRON_FIELD_DEFS[field];
  if (!opts) return value;
  const found = opts.find((o) => o.value.toLowerCase() === value.toLowerCase());
  return found ? found.title : value;
}
const DEFAULT_BUILDER = {
  second: "*",
  minute: "*",
  hour: "*",
  day: "*",
  month: "*",
  dow: "*"
};
function builderToExpr(b) {
  return `${b.second} ${b.minute} ${b.hour} ${b.day} ${b.month} ${b.dow}`;
}
function exprToBuilder(expr) {
  const fields = expr.trim().split(/\s+/);
  if (fields.length !== 6) return { ...DEFAULT_BUILDER };
  return {
    second: fields[0],
    minute: fields[1],
    hour: fields[2],
    day: fields[3],
    month: fields[4],
    dow: fields[5]
  };
}
const _hoisted_1 = { class: "crons-page" };
const _hoisted_2 = { class: "page-header d-flex align-center justify-space-between mb-6" };
const _hoisted_3 = { class: "d-flex align-center" };
const _hoisted_4 = { class: "header-icon-wrapper mr-3" };
const _hoisted_5 = {
  key: 0,
  class: "stats-bar d-flex align-center pa-4 mb-6 rounded-lg"
};
const _hoisted_6 = { class: "stat-item d-flex align-center" };
const _hoisted_7 = { class: "text-body-2 font-weight-medium" };
const _hoisted_8 = { class: "stat-item d-flex align-center" };
const _hoisted_9 = { class: "text-body-2 font-weight-medium" };
const _hoisted_10 = { class: "stat-item d-flex align-center" };
const _hoisted_11 = { class: "text-body-2 font-weight-medium" };
const _hoisted_12 = { class: "info-grid" };
const _hoisted_13 = { class: "info-item info-item-wide" };
const _hoisted_14 = { class: "cron-expr-badge" };
const _hoisted_15 = { class: "info-item" };
const _hoisted_16 = { class: "info-value" };
const _hoisted_17 = { class: "info-item" };
const _hoisted_18 = { class: "info-value" };
const _hoisted_19 = { class: "info-item" };
const _hoisted_20 = { class: "info-value d-flex align-center" };
const _hoisted_21 = {
  key: 0,
  class: "info-item info-item-wide mt-2"
};
const _hoisted_22 = { class: "info-value text-truncate" };
const _hoisted_23 = {
  key: 2,
  class: "empty-state text-center py-12"
};
const _hoisted_24 = { class: "empty-icon-wrapper mb-6" };
const _hoisted_25 = { class: "text-h6 font-weight-bold" };
const _hoisted_26 = { class: "text-caption text-medium-emphasis mt-1" };
const _hoisted_27 = { class: "dialog-section" };
const _hoisted_28 = { class: "section-header d-flex align-center mb-3" };
const _hoisted_29 = { class: "dialog-section schedule-section" };
const _hoisted_30 = { class: "section-header d-flex align-center mb-3" };
const _hoisted_31 = {
  key: 0,
  class: "cron-builder-panel pa-3 rounded-lg mb-3"
};
const _hoisted_32 = { class: "builder-row" };
const _hoisted_33 = { class: "builder-field" };
const _hoisted_34 = { class: "builder-field" };
const _hoisted_35 = { class: "builder-field" };
const _hoisted_36 = { class: "builder-row" };
const _hoisted_37 = { class: "builder-field" };
const _hoisted_38 = { class: "builder-field" };
const _hoisted_39 = { class: "builder-field" };
const _hoisted_40 = {
  key: 0,
  class: "cron-description pa-2 rounded-lg mt-2 mb-2"
};
const _hoisted_41 = { class: "text-caption text-medium-emphasis" };
const _hoisted_42 = {
  key: 0,
  class: "next-runs-panel pa-3 rounded-lg mt-3"
};
const _hoisted_43 = { class: "d-flex align-center" };
const _hoisted_44 = { class: "next-runs-icon-wrapper mr-2" };
const _hoisted_45 = { class: "d-flex align-center mt-2" };
const _hoisted_46 = { class: "text-body-2 text-medium-emphasis" };
const _hoisted_47 = {
  key: 0,
  class: "schedule-helper-section mb-2"
};
const _hoisted_48 = { class: "preset-grid" };
const _hoisted_49 = ["onClick"];
const _hoisted_50 = { class: "preset-label text-caption" };
const _hoisted_51 = { class: "preset-expr text-caption" };
const _hoisted_52 = { class: "dialog-section" };
const _hoisted_53 = { class: "section-header d-flex align-center mb-3" };
const _hoisted_54 = { class: "enabled-toggle" };
const _hoisted_55 = { class: "text-body-1" };
const _hoisted_56 = { class: "text-error" };
const _hoisted_57 = { class: "d-flex align-center mb-4" };
const _hoisted_58 = {
  key: 0,
  class: "text-caption text-medium-emphasis"
};
const _hoisted_59 = {
  key: 0,
  class: "text-center pa-8"
};
const _hoisted_60 = { class: "text-body-2 font-weight-medium" };
const _hoisted_61 = {
  key: 2,
  class: "text-center py-8"
};
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "CronsPage",
  setup(__props) {
    const cronStore = useCronStore();
    const appStore = useAppStore();
    const availableModels = ref([]);
    const showDialog = ref(false);
    const editingCron = ref(null);
    const saving = ref(false);
    const deleteDialog = ref(false);
    const deleteTarget = ref(null);
    const deleting = ref(false);
    const historyDialog = ref(false);
    const historyTarget = ref(null);
    const historyLogs = ref([]);
    const historyLoading = ref(false);
    const nameError = ref("");
    const cronError = ref("");
    const cronFocused = ref(false);
    const cronBuilder = ref({ ...DEFAULT_BUILDER });
    const cronBuildMode = ref(false);
    const cronPresets = [
      { label: "Every minute", expr: "0 * * * * *" },
      { label: "Every 5 min", expr: "0 */5 * * * *" },
      { label: "Every 15 min", expr: "0 */15 * * * *" },
      { label: "Every hour", expr: "0 0 * * * *" },
      { label: "Daily midnight", expr: "0 0 0 * * *" },
      { label: "Daily 6 AM", expr: "0 0 6 * * *" },
      { label: "Weekends 6 AM", expr: "0 0 6 * * Sun,Sat" },
      { label: "Monthly 1st", expr: "0 0 0 1 * *" },
      { label: "Weekdays 9 AM", expr: "0 0 9 * * Mon-Fri" }
    ];
    const defaultForm = () => ({
      name: "",
      description: "",
      cron: "",
      enabled: true,
      timeout: null,
      prompt: null,
      model: null
    });
    const form = ref(defaultForm());
    const nextRuns = computed(() => {
      const expr = form.value.cron.trim();
      if (!expr || !isValidCronExpr(expr)) {
        return [];
      }
      return getNextRunTimes(expr, 1);
    });
    const cronDescription = computed(() => {
      const expr = form.value.cron.trim();
      if (!expr || !isValidCronExpr(expr)) return "";
      return describeCron(expr);
    });
    function syncBuilderFromExpr(expr) {
      const trimmed = expr.trim();
      if (isValidCronExpr(trimmed)) {
        cronBuilder.value = exprToBuilder(trimmed);
      }
    }
    function applyBuilderToExpr() {
      form.value.cron = builderToExpr(cronBuilder.value);
      cronError.value = "";
    }
    function onCronInput() {
      cronError.value = "";
      syncBuilderFromExpr(form.value.cron);
    }
    function onBuilderFieldChange() {
      applyBuilderToExpr();
    }
    function toggleBuildMode() {
      cronBuildMode.value = !cronBuildMode.value;
      if (cronBuildMode.value) {
        syncBuilderFromExpr(form.value.cron);
      }
    }
    function validateName() {
      const trimmed = form.value.name.trim();
      if (!trimmed) {
        nameError.value = "Name is required";
        return false;
      }
      const existing = cronStore.crons.find(
        (c) => {
          var _a;
          return c.name.toLowerCase() === trimmed.toLowerCase() && c.id !== ((_a = editingCron.value) == null ? void 0 : _a.id);
        }
      );
      if (existing) {
        nameError.value = "A cron job with this name already exists";
        return false;
      }
      return true;
    }
    function validateCron() {
      if (!form.value.cron.trim()) {
        cronError.value = "Cron expression is required";
        return false;
      }
      if (!isValidCronExpr(form.value.cron.trim())) {
        cronError.value = "Must have 6 fields: sec min hour day month day_of_week";
        return false;
      }
      return true;
    }
    function onCronBlur() {
      nextTick(() => {
        cronFocused.value = false;
      });
    }
    function openAddDialog() {
      editingCron.value = null;
      form.value = defaultForm();
      nameError.value = "";
      cronError.value = "";
      cronFocused.value = false;
      cronBuildMode.value = false;
      cronBuilder.value = { ...DEFAULT_BUILDER };
      showDialog.value = true;
    }
    function openEditDialog(cron) {
      editingCron.value = cron;
      form.value = {
        name: cron.name,
        description: cron.description,
        cron: cron.cron,
        enabled: cron.enabled,
        timeout: cron.timeout,
        prompt: cron.prompt,
        model: cron.model
      };
      nameError.value = "";
      cronError.value = "";
      cronFocused.value = false;
      cronBuildMode.value = false;
      syncBuilderFromExpr(cron.cron);
      showDialog.value = true;
    }
    async function handleSave() {
      nameError.value = "";
      cronError.value = "";
      if (!validateName()) return;
      if (!validateCron()) return;
      saving.value = true;
      try {
        if (editingCron.value) {
          await cronStore.updateCron(editingCron.value.id, { ...form.value });
          appStore.showMessage("Cron updated successfully", "success");
        } else {
          await cronStore.addCron({ ...form.value });
          appStore.showMessage("Cron created successfully", "success");
        }
        showDialog.value = false;
        form.value = defaultForm();
        editingCron.value = null;
      } catch {
        appStore.showMessage("Failed to save cron job", "error");
      } finally {
        saving.value = false;
      }
    }
    async function handleDelete() {
      if (!deleteTarget.value) return;
      deleting.value = true;
      try {
        await cronStore.removeCron(deleteTarget.value.id);
        appStore.showMessage("Cron deleted", "success");
      } catch {
        appStore.showMessage("Failed to delete cron", "error");
      } finally {
        deleting.value = false;
        deleteDialog.value = false;
        deleteTarget.value = null;
      }
    }
    async function handleToggleEnabled(cron, enabled) {
      try {
        await cronStore.updateCron(cron.id, { enabled });
        appStore.showMessage(enabled ? "Cron enabled" : "Cron disabled", "success");
      } catch {
        appStore.showMessage("Failed to update cron", "error");
      }
    }
    async function openHistory(cron) {
      historyTarget.value = cron;
      historyDialog.value = true;
      historyLoading.value = true;
      try {
        historyLogs.value = await cronStore.fetchCronLogs(cron.id);
      } catch {
        historyLogs.value = [];
      } finally {
        historyLoading.value = false;
      }
    }
    async function refreshHistory() {
      if (!historyTarget.value) return;
      historyLoading.value = true;
      try {
        historyLogs.value = await cronStore.fetchCronLogs(historyTarget.value.id);
      } catch {
        historyLogs.value = [];
      } finally {
        historyLoading.value = false;
      }
    }
    function formatDate(dateStr) {
      return new Date(dateStr).toLocaleDateString("zh-CN", {
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit"
      });
    }
    function formatNextRunTime(date) {
      return date.toLocaleDateString("zh-CN", {
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit"
      });
    }
    function nextRunRelative(date) {
      const now = /* @__PURE__ */ new Date();
      const diffMs = date.getTime() - now.getTime();
      const diffMin = Math.round(diffMs / 6e4);
      const diffHr = Math.round(diffMs / 36e5);
      const diffDay = Math.round(diffMs / 864e5);
      if (diffMin < 1) return "now";
      if (diffMin < 60) return `in ${diffMin}m`;
      if (diffHr < 24) return `in ${diffHr}h`;
      return `in ${diffDay}d`;
    }
    onMounted(() => {
      cronStore.fetchCrons();
      api.getModels().then((m) => availableModels.value = m).catch(() => availableModels.value = []);
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", _hoisted_1, [
        createBaseVNode("div", _hoisted_2, [
          createBaseVNode("div", _hoisted_3, [
            createBaseVNode("div", _hoisted_4, [
              createVNode(VIcon, {
                size: "28",
                color: "white"
              }, {
                default: withCtx(() => [..._cache[23] || (_cache[23] = [
                  createTextVNode("mdi-clock-outline", -1)
                ])]),
                _: 1
              })
            ]),
            _cache[24] || (_cache[24] = createBaseVNode("div", null, [
              createBaseVNode("h1", { class: "text-h4 font-weight-bold mb-0" }, "Cron Jobs"),
              createBaseVNode("p", { class: "text-body-2 text-medium-emphasis mt-1" }, "Manage scheduled AI agent tasks")
            ], -1))
          ]),
          createVNode(VBtn, {
            color: "primary",
            size: "large",
            "prepend-icon": "mdi-plus",
            variant: "elevated",
            class: "add-btn",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[25] || (_cache[25] = [
              createTextVNode(" Add Cron ", -1)
            ])]),
            _: 1
          })
        ]),
        !unref(cronStore).loading && unref(cronStore).crons.length > 0 ? (openBlock(), createElementBlock("div", _hoisted_5, [
          createBaseVNode("div", _hoisted_6, [
            createVNode(VIcon, {
              size: "20",
              color: "primary",
              class: "mr-2"
            }, {
              default: withCtx(() => [..._cache[26] || (_cache[26] = [
                createTextVNode("mdi-clock-outline", -1)
              ])]),
              _: 1
            }),
            createBaseVNode("span", _hoisted_7, toDisplayString(unref(cronStore).crons.length) + " job" + toDisplayString(unref(cronStore).crons.length > 1 ? "s" : ""), 1)
          ]),
          createVNode(VDivider, {
            vertical: "",
            class: "mx-4"
          }),
          createBaseVNode("div", _hoisted_8, [
            createVNode(VIcon, {
              size: "20",
              color: "success",
              class: "mr-2"
            }, {
              default: withCtx(() => [..._cache[27] || (_cache[27] = [
                createTextVNode("mdi-play-circle", -1)
              ])]),
              _: 1
            }),
            createBaseVNode("span", _hoisted_9, toDisplayString(unref(cronStore).crons.filter((c) => c.enabled).length) + " running", 1)
          ]),
          createVNode(VDivider, {
            vertical: "",
            class: "mx-4"
          }),
          createBaseVNode("div", _hoisted_10, [
            createVNode(VIcon, {
              size: "20",
              color: "medium-emphasis",
              class: "mr-2"
            }, {
              default: withCtx(() => [..._cache[28] || (_cache[28] = [
                createTextVNode("mdi-pause-circle", -1)
              ])]),
              _: 1
            }),
            createBaseVNode("span", _hoisted_11, toDisplayString(unref(cronStore).crons.filter((c) => !c.enabled).length) + " paused", 1)
          ])
        ])) : createCommentVNode("", true),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(cronStore).crons, (cron) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                md: "6",
                lg: "4",
                key: cron.id
              }, {
                default: withCtx(() => [
                  createVNode(VCard, {
                    class: normalizeClass(["cron-card fill-height", { "cron-disabled": !cron.enabled }]),
                    elevation: "2"
                  }, {
                    default: withCtx(() => [
                      createBaseVNode("div", {
                        class: normalizeClass(["card-top-bar", cron.enabled ? "bg-primary" : "bg-grey"])
                      }, null, 2),
                      createVNode(VCardItem, { class: "pb-1" }, {
                        prepend: withCtx(() => [
                          createVNode(VAvatar, {
                            color: cron.enabled ? "primary" : "grey",
                            size: "40",
                            variant: "tonal"
                          }, {
                            default: withCtx(() => [
                              createVNode(VIcon, {
                                color: cron.enabled ? "primary" : "grey-darken-1",
                                size: "22"
                              }, {
                                default: withCtx(() => [..._cache[29] || (_cache[29] = [
                                  createTextVNode(" mdi-clock-outline ", -1)
                                ])]),
                                _: 1
                              }, 8, ["color"])
                            ]),
                            _: 2
                          }, 1032, ["color"])
                        ]),
                        append: withCtx(() => [
                          createVNode(VChip, {
                            color: cron.enabled ? "success" : "medium-emphasis",
                            size: "x-small",
                            variant: "tonal",
                            label: ""
                          }, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(cron.enabled ? "Running" : "Paused"), 1)
                            ]),
                            _: 2
                          }, 1032, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, { class: "text-body-1 font-weight-bold pr-2" }, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(cron.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          cron.description ? (openBlock(), createBlock(VCardSubtitle, {
                            key: 0,
                            class: "mt-1"
                          }, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(cron.description), 1)
                            ]),
                            _: 2
                          }, 1024)) : createCommentVNode("", true)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, { class: "pt-2" }, {
                        default: withCtx(() => [
                          createBaseVNode("div", _hoisted_12, [
                            createBaseVNode("div", _hoisted_13, [
                              _cache[30] || (_cache[30] = createBaseVNode("span", { class: "info-label" }, "Cron", -1)),
                              createBaseVNode("code", _hoisted_14, toDisplayString(cron.cron), 1)
                            ]),
                            createBaseVNode("div", _hoisted_15, [
                              _cache[31] || (_cache[31] = createBaseVNode("span", { class: "info-label" }, "Timeout", -1)),
                              createBaseVNode("span", _hoisted_16, toDisplayString(cron.timeout ? `${cron.timeout}s` : "-"), 1)
                            ]),
                            createBaseVNode("div", _hoisted_17, [
                              _cache[32] || (_cache[32] = createBaseVNode("span", { class: "info-label" }, "Model", -1)),
                              createBaseVNode("span", _hoisted_18, toDisplayString(cron.model || "Default"), 1)
                            ]),
                            createBaseVNode("div", _hoisted_19, [
                              _cache[35] || (_cache[35] = createBaseVNode("span", { class: "info-label" }, "Last Run", -1)),
                              createBaseVNode("span", _hoisted_20, [
                                cron.last_run_at ? (openBlock(), createElementBlock(Fragment, { key: 0 }, [
                                  createVNode(VIcon, {
                                    color: cron.last_run_success ? "success" : "error",
                                    size: "14",
                                    class: "mr-1"
                                  }, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(cron.last_run_success ? "mdi-check-circle" : "mdi-alert-circle"), 1)
                                    ]),
                                    _: 2
                                  }, 1032, ["color"]),
                                  createTextVNode(" " + toDisplayString(formatDate(cron.last_run_at)), 1)
                                ], 64)) : (openBlock(), createElementBlock(Fragment, { key: 1 }, [
                                  createVNode(VIcon, {
                                    size: "14",
                                    color: "medium-emphasis",
                                    class: "mr-1"
                                  }, {
                                    default: withCtx(() => [..._cache[33] || (_cache[33] = [
                                      createTextVNode("mdi-minus-circle", -1)
                                    ])]),
                                    _: 1
                                  }),
                                  _cache[34] || (_cache[34] = createTextVNode(" Never ", -1))
                                ], 64))
                              ])
                            ])
                          ]),
                          cron.prompt ? (openBlock(), createElementBlock("div", _hoisted_21, [
                            _cache[36] || (_cache[36] = createBaseVNode("span", { class: "info-label" }, "Prompt", -1)),
                            createBaseVNode("span", _hoisted_22, toDisplayString(cron.prompt), 1)
                          ])) : createCommentVNode("", true)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, { class: "px-4 pb-3" }, {
                        default: withCtx(() => [
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-history",
                            onClick: withModifiers(($event) => openHistory(cron), ["stop"])
                          }, {
                            default: withCtx(() => [..._cache[37] || (_cache[37] = [
                              createTextVNode(" History ", -1)
                            ])]),
                            _: 1
                          }, 8, ["onClick"]),
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-pencil",
                            onClick: ($event) => openEditDialog(cron)
                          }, {
                            default: withCtx(() => [..._cache[38] || (_cache[38] = [
                              createTextVNode(" Edit ", -1)
                            ])]),
                            _: 1
                          }, 8, ["onClick"]),
                          createVNode(VSpacer),
                          createVNode(VSwitch, {
                            "model-value": cron.enabled,
                            color: "success",
                            density: "compact",
                            "hide-details": "",
                            class: "mr-1",
                            "onUpdate:modelValue": (v) => handleToggleEnabled(cron, !!v)
                          }, null, 8, ["model-value", "onUpdate:modelValue"])
                        ]),
                        _: 2
                      }, 1024)
                    ]),
                    _: 2
                  }, 1032, ["class"])
                ]),
                _: 2
              }, 1024);
            }), 128))
          ]),
          _: 1
        }),
        unref(cronStore).loading ? (openBlock(), createBlock(VSkeletonLoader, {
          key: 1,
          type: "card@3"
        })) : createCommentVNode("", true),
        !unref(cronStore).loading && unref(cronStore).crons.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_23, [
          createBaseVNode("div", _hoisted_24, [
            createVNode(VIcon, {
              size: "72",
              color: "primary"
            }, {
              default: withCtx(() => [..._cache[39] || (_cache[39] = [
                createTextVNode("mdi-clock-off", -1)
              ])]),
              _: 1
            })
          ]),
          _cache[41] || (_cache[41] = createBaseVNode("h3", { class: "text-h5 font-weight-medium mb-2" }, "No cron jobs configured", -1)),
          _cache[42] || (_cache[42] = createBaseVNode("p", { class: "text-body-1 text-medium-emphasis mb-6" }, "Schedule recurring AI agent tasks with cron expressions", -1)),
          createVNode(VBtn, {
            color: "primary",
            size: "large",
            variant: "elevated",
            "prepend-icon": "mdi-plus",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[40] || (_cache[40] = [
              createTextVNode(" Create Your First Cron ", -1)
            ])]),
            _: 1
          })
        ])) : createCommentVNode("", true),
        createVNode(VDialog, {
          modelValue: showDialog.value,
          "onUpdate:modelValue": _cache[18] || (_cache[18] = ($event) => showDialog.value = $event),
          "max-width": "820",
          transition: "dialog-bottom-transition"
        }, {
          default: withCtx(() => [
            createVNode(VCard, { class: "dialog-card" }, {
              default: withCtx(() => [
                createBaseVNode("div", {
                  class: normalizeClass(["dialog-top-bar dialog-top-bar-thick", editingCron.value ? "bg-warning" : "bg-primary"])
                }, null, 2),
                createVNode(VCardTitle, { class: "d-flex align-center pt-5 px-6" }, {
                  default: withCtx(() => [
                    createBaseVNode("div", {
                      class: normalizeClass(["dialog-title-icon mr-3", editingCron.value ? "bg-warning" : "bg-primary"])
                    }, [
                      createVNode(VIcon, {
                        color: "white",
                        size: "22"
                      }, {
                        default: withCtx(() => [
                          createTextVNode(toDisplayString(editingCron.value ? "mdi-pencil" : "mdi-plus"), 1)
                        ]),
                        _: 1
                      })
                    ], 2),
                    createBaseVNode("div", null, [
                      createBaseVNode("div", _hoisted_25, toDisplayString(editingCron.value ? "Edit Cron Job" : "New Cron Job"), 1),
                      createBaseVNode("div", _hoisted_26, toDisplayString(editingCron.value ? "Modify schedule, prompt, and model settings" : "Configure a new recurring AI agent task"), 1)
                    ])
                  ]),
                  _: 1
                }),
                createVNode(VCardText, { class: "px-6" }, {
                  default: withCtx(() => [
                    createBaseVNode("div", _hoisted_27, [
                      createBaseVNode("div", _hoisted_28, [
                        createVNode(VIcon, {
                          size: "18",
                          color: "primary",
                          class: "mr-2"
                        }, {
                          default: withCtx(() => [..._cache[43] || (_cache[43] = [
                            createTextVNode("mdi-information-outline", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[44] || (_cache[44] = createBaseVNode("span", { class: "text-body-2 font-weight-bold" }, "Basic Information", -1))
                      ]),
                      createVNode(VRow, { dense: "" }, {
                        default: withCtx(() => [
                          createVNode(VCol, {
                            cols: "12",
                            md: "7"
                          }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.name,
                                "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => form.value.name = $event),
                                label: "Name *",
                                variant: "outlined",
                                placeholder: "daily-report",
                                "error-messages": nameError.value,
                                density: "comfortable",
                                onInput: _cache[1] || (_cache[1] = ($event) => nameError.value = "")
                              }, null, 8, ["modelValue", "error-messages"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, {
                            cols: "12",
                            md: "5"
                          }, {
                            default: withCtx(() => [
                              createVNode(VSelect, {
                                modelValue: form.value.model,
                                "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => form.value.model = $event),
                                label: "Model",
                                items: availableModels.value,
                                variant: "outlined",
                                clearable: "",
                                placeholder: "System default",
                                density: "comfortable"
                              }, null, 8, ["modelValue", "items"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, { cols: "12" }, {
                            default: withCtx(() => [
                              createVNode(VTextarea, {
                                modelValue: form.value.description,
                                "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => form.value.description = $event),
                                label: "Description",
                                variant: "outlined",
                                rows: "2",
                                placeholder: "Describe what this cron job does...",
                                density: "comfortable"
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          })
                        ]),
                        _: 1
                      })
                    ]),
                    createBaseVNode("div", _hoisted_29, [
                      createBaseVNode("div", _hoisted_30, [
                        createVNode(VIcon, {
                          size: "18",
                          color: "secondary",
                          class: "mr-2"
                        }, {
                          default: withCtx(() => [..._cache[45] || (_cache[45] = [
                            createTextVNode("mdi-calendar-sync", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[46] || (_cache[46] = createBaseVNode("span", { class: "text-body-2 font-weight-bold" }, "Schedule", -1)),
                        createVNode(VSpacer),
                        createVNode(VBtn, {
                          variant: "text",
                          size: "x-small",
                          color: cronBuildMode.value ? "secondary" : "medium-emphasis",
                          "prepend-icon": cronBuildMode.value ? "mdi-code-tags" : "mdi-tune-variant",
                          onClick: toggleBuildMode
                        }, {
                          default: withCtx(() => [
                            createTextVNode(toDisplayString(cronBuildMode.value ? "Expression" : "Builder"), 1)
                          ]),
                          _: 1
                        }, 8, ["color", "prepend-icon"])
                      ]),
                      createVNode(VExpandTransition, null, {
                        default: withCtx(() => [
                          cronBuildMode.value ? (openBlock(), createElementBlock("div", _hoisted_31, [
                            createBaseVNode("div", _hoisted_32, [
                              createBaseVNode("div", _hoisted_33, [
                                _cache[47] || (_cache[47] = createBaseVNode("label", { class: "builder-label" }, "Second", -1)),
                                createVNode(VSelect, {
                                  "model-value": cronBuilder.value.second,
                                  items: unref(CRON_FIELD_DEFS).second,
                                  "item-title": "title",
                                  "item-value": "value",
                                  variant: "outlined",
                                  density: "compact",
                                  "hide-details": "",
                                  "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => {
                                    cronBuilder.value.second = $event;
                                    onBuilderFieldChange();
                                  })
                                }, null, 8, ["model-value", "items"])
                              ]),
                              createBaseVNode("div", _hoisted_34, [
                                _cache[48] || (_cache[48] = createBaseVNode("label", { class: "builder-label" }, "Minute", -1)),
                                createVNode(VSelect, {
                                  "model-value": cronBuilder.value.minute,
                                  items: unref(CRON_FIELD_DEFS).minute,
                                  "item-title": "title",
                                  "item-value": "value",
                                  variant: "outlined",
                                  density: "compact",
                                  "hide-details": "",
                                  "onUpdate:modelValue": _cache[5] || (_cache[5] = ($event) => {
                                    cronBuilder.value.minute = $event;
                                    onBuilderFieldChange();
                                  })
                                }, null, 8, ["model-value", "items"])
                              ]),
                              createBaseVNode("div", _hoisted_35, [
                                _cache[49] || (_cache[49] = createBaseVNode("label", { class: "builder-label" }, "Hour", -1)),
                                createVNode(VSelect, {
                                  "model-value": cronBuilder.value.hour,
                                  items: unref(CRON_FIELD_DEFS).hour,
                                  "item-title": "title",
                                  "item-value": "value",
                                  variant: "outlined",
                                  density: "compact",
                                  "hide-details": "",
                                  "onUpdate:modelValue": _cache[6] || (_cache[6] = ($event) => {
                                    cronBuilder.value.hour = $event;
                                    onBuilderFieldChange();
                                  })
                                }, null, 8, ["model-value", "items"])
                              ])
                            ]),
                            createBaseVNode("div", _hoisted_36, [
                              createBaseVNode("div", _hoisted_37, [
                                _cache[50] || (_cache[50] = createBaseVNode("label", { class: "builder-label" }, "Day of Month", -1)),
                                createVNode(VSelect, {
                                  "model-value": cronBuilder.value.day,
                                  items: unref(CRON_FIELD_DEFS).day,
                                  "item-title": "title",
                                  "item-value": "value",
                                  variant: "outlined",
                                  density: "compact",
                                  "hide-details": "",
                                  "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => {
                                    cronBuilder.value.day = $event;
                                    onBuilderFieldChange();
                                  })
                                }, null, 8, ["model-value", "items"])
                              ]),
                              createBaseVNode("div", _hoisted_38, [
                                _cache[51] || (_cache[51] = createBaseVNode("label", { class: "builder-label" }, "Month", -1)),
                                createVNode(VSelect, {
                                  "model-value": cronBuilder.value.month,
                                  items: unref(CRON_FIELD_DEFS).month,
                                  "item-title": "title",
                                  "item-value": "value",
                                  variant: "outlined",
                                  density: "compact",
                                  "hide-details": "",
                                  "onUpdate:modelValue": _cache[8] || (_cache[8] = ($event) => {
                                    cronBuilder.value.month = $event;
                                    onBuilderFieldChange();
                                  })
                                }, null, 8, ["model-value", "items"])
                              ]),
                              createBaseVNode("div", _hoisted_39, [
                                _cache[52] || (_cache[52] = createBaseVNode("label", { class: "builder-label" }, "Day of Week", -1)),
                                createVNode(VSelect, {
                                  "model-value": cronBuilder.value.dow,
                                  items: unref(CRON_FIELD_DEFS).dow,
                                  "item-title": "title",
                                  "item-value": "value",
                                  variant: "outlined",
                                  density: "compact",
                                  "hide-details": "",
                                  "onUpdate:modelValue": _cache[9] || (_cache[9] = ($event) => {
                                    cronBuilder.value.dow = $event;
                                    onBuilderFieldChange();
                                  })
                                }, null, 8, ["model-value", "items"])
                              ])
                            ])
                          ])) : createCommentVNode("", true)
                        ]),
                        _: 1
                      }),
                      createVNode(VRow, { dense: "" }, {
                        default: withCtx(() => [
                          createVNode(VCol, { cols: "12" }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.cron,
                                "onUpdate:modelValue": _cache[10] || (_cache[10] = ($event) => form.value.cron = $event),
                                label: "Cron Expression *",
                                variant: "outlined",
                                placeholder: cronBuildMode.value ? "auto-generated from builder" : "0 0 9 * * Mon-Fri",
                                hint: "Click to edit — presets and next-run preview will appear",
                                "persistent-hint": "",
                                density: "comfortable",
                                "error-messages": cronError.value,
                                readonly: cronBuildMode.value,
                                onInput: _cache[11] || (_cache[11] = ($event) => onCronInput()),
                                onFocus: _cache[12] || (_cache[12] = ($event) => cronFocused.value = true),
                                onBlur: onCronBlur
                              }, {
                                "prepend-inner": withCtx(() => [
                                  createVNode(VIcon, {
                                    size: "18",
                                    color: "medium-emphasis"
                                  }, {
                                    default: withCtx(() => [..._cache[53] || (_cache[53] = [
                                      createTextVNode("mdi-code-tags", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                _: 1
                              }, 8, ["modelValue", "placeholder", "error-messages", "readonly"])
                            ]),
                            _: 1
                          })
                        ]),
                        _: 1
                      }),
                      cronDescription.value ? (openBlock(), createElementBlock("div", _hoisted_40, [
                        createVNode(VIcon, {
                          size: "14",
                          color: "medium-emphasis",
                          class: "mr-1"
                        }, {
                          default: withCtx(() => [..._cache[54] || (_cache[54] = [
                            createTextVNode("mdi-text-short", -1)
                          ])]),
                          _: 1
                        }),
                        createBaseVNode("span", _hoisted_41, toDisplayString(cronDescription.value), 1)
                      ])) : createCommentVNode("", true),
                      createVNode(VExpandTransition, null, {
                        default: withCtx(() => [
                          cronFocused.value && nextRuns.value.length > 0 ? (openBlock(), createElementBlock("div", _hoisted_42, [
                            createBaseVNode("div", _hoisted_43, [
                              createBaseVNode("div", _hoisted_44, [
                                createVNode(VIcon, {
                                  size: "16",
                                  color: "primary"
                                }, {
                                  default: withCtx(() => [..._cache[55] || (_cache[55] = [
                                    createTextVNode("mdi-calendar-clock", -1)
                                  ])]),
                                  _: 1
                                })
                              ]),
                              _cache[56] || (_cache[56] = createBaseVNode("span", { class: "text-caption font-weight-bold text-primary" }, "Next Execution", -1)),
                              createVNode(VSpacer),
                              createVNode(VChip, {
                                size: "x-small",
                                color: "primary",
                                variant: "flat",
                                label: ""
                              }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(nextRunRelative(nextRuns.value[0])), 1)
                                ]),
                                _: 1
                              })
                            ]),
                            createBaseVNode("div", _hoisted_45, [
                              createVNode(VIcon, {
                                size: "14",
                                color: "medium-emphasis",
                                class: "mr-1"
                              }, {
                                default: withCtx(() => [..._cache[57] || (_cache[57] = [
                                  createTextVNode("mdi-clock-outline", -1)
                                ])]),
                                _: 1
                              }),
                              createBaseVNode("span", _hoisted_46, toDisplayString(formatNextRunTime(nextRuns.value[0])) + " UTC", 1)
                            ])
                          ])) : createCommentVNode("", true)
                        ]),
                        _: 1
                      }),
                      createVNode(VDivider, { class: "my-3" }),
                      createVNode(VExpandTransition, null, {
                        default: withCtx(() => [
                          cronFocused.value ? (openBlock(), createElementBlock("div", _hoisted_47, [
                            createBaseVNode("div", _hoisted_48, [
                              (openBlock(), createElementBlock(Fragment, null, renderList(cronPresets, (preset) => {
                                return createBaseVNode("div", {
                                  key: preset.label,
                                  class: normalizeClass(["preset-chip", { "preset-active": form.value.cron === preset.expr }]),
                                  onMousedown: _cache[13] || (_cache[13] = withModifiers(() => {
                                  }, ["prevent"])),
                                  onClick: ($event) => {
                                    form.value.cron = preset.expr;
                                    cronError.value = "";
                                    syncBuilderFromExpr(preset.expr);
                                  }
                                }, [
                                  createBaseVNode("span", _hoisted_50, toDisplayString(preset.label), 1),
                                  createBaseVNode("code", _hoisted_51, toDisplayString(preset.expr), 1)
                                ], 42, _hoisted_49);
                              }), 64))
                            ])
                          ])) : createCommentVNode("", true)
                        ]),
                        _: 1
                      }),
                      createVNode(VExpandTransition, null, {
                        default: withCtx(() => [
                          cronFocused.value ? (openBlock(), createBlock(VAlert, {
                            key: 0,
                            type: "info",
                            variant: "tonal",
                            density: "compact",
                            class: "syntax-alert mt-0"
                          }, {
                            text: withCtx(() => [..._cache[58] || (_cache[58] = [
                              createBaseVNode("div", { class: "syntax-grid" }, [
                                createBaseVNode("div", { class: "syntax-item" }, [
                                  createBaseVNode("code", { class: "syntax-expr" }, "0 2,14,26 * * * *"),
                                  createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "Comma values")
                                ]),
                                createBaseVNode("div", { class: "syntax-item" }, [
                                  createBaseVNode("code", { class: "syntax-expr" }, "0 0 * 5-10 * *"),
                                  createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "Ranges")
                                ]),
                                createBaseVNode("div", { class: "syntax-item" }, [
                                  createBaseVNode("code", { class: "syntax-expr" }, "0 */15 * * * *"),
                                  createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "Steps")
                                ]),
                                createBaseVNode("div", { class: "syntax-item" }, [
                                  createBaseVNode("code", { class: "syntax-expr" }, "0 0 6 * * Sun,Sat"),
                                  createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "Day names")
                                ])
                              ], -1)
                            ])]),
                            _: 1
                          })) : createCommentVNode("", true)
                        ]),
                        _: 1
                      })
                    ]),
                    createBaseVNode("div", _hoisted_52, [
                      createBaseVNode("div", _hoisted_53, [
                        createVNode(VIcon, {
                          size: "18",
                          color: "tertiary",
                          class: "mr-2"
                        }, {
                          default: withCtx(() => [..._cache[59] || (_cache[59] = [
                            createTextVNode("mdi-message-text-outline", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[60] || (_cache[60] = createBaseVNode("span", { class: "text-body-2 font-weight-bold" }, "Task", -1))
                      ]),
                      createVNode(VRow, { dense: "" }, {
                        default: withCtx(() => [
                          createVNode(VCol, { cols: "12" }, {
                            default: withCtx(() => [
                              createVNode(VTextarea, {
                                modelValue: form.value.prompt,
                                "onUpdate:modelValue": _cache[14] || (_cache[14] = ($event) => form.value.prompt = $event),
                                label: "Prompt",
                                variant: "outlined",
                                rows: "4",
                                placeholder: "Enter the AI prompt to execute on schedule...",
                                density: "comfortable",
                                "auto-grow": ""
                              }, null, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, {
                            cols: "6",
                            md: "4"
                          }, {
                            default: withCtx(() => [
                              createVNode(VTextField, {
                                modelValue: form.value.timeout,
                                "onUpdate:modelValue": _cache[15] || (_cache[15] = ($event) => form.value.timeout = $event),
                                modelModifiers: { number: true },
                                label: "Timeout (s)",
                                type: "number",
                                variant: "outlined",
                                placeholder: "30",
                                density: "comfortable",
                                min: "1"
                              }, {
                                "prepend-inner": withCtx(() => [
                                  createVNode(VIcon, {
                                    size: "16",
                                    color: "medium-emphasis"
                                  }, {
                                    default: withCtx(() => [..._cache[61] || (_cache[61] = [
                                      createTextVNode("mdi-timer-outline", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                _: 1
                              }, 8, ["modelValue"])
                            ]),
                            _: 1
                          }),
                          createVNode(VCol, {
                            cols: "6",
                            md: "4"
                          }, {
                            default: withCtx(() => [
                              createBaseVNode("div", _hoisted_54, [
                                _cache[62] || (_cache[62] = createBaseVNode("span", { class: "text-caption text-medium-emphasis mr-2" }, "Enabled", -1)),
                                createVNode(VSwitch, {
                                  modelValue: form.value.enabled,
                                  "onUpdate:modelValue": _cache[16] || (_cache[16] = ($event) => form.value.enabled = $event),
                                  color: "success",
                                  "hide-details": "",
                                  density: "compact"
                                }, null, 8, ["modelValue"])
                              ])
                            ]),
                            _: 1
                          })
                        ]),
                        _: 1
                      })
                    ])
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, { class: "dialog-actions px-6 pb-5 pt-2" }, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "outlined",
                      size: "large",
                      onClick: _cache[17] || (_cache[17] = ($event) => showDialog.value = false),
                      class: "px-6"
                    }, {
                      default: withCtx(() => [..._cache[63] || (_cache[63] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      variant: "elevated",
                      size: "large",
                      onClick: handleSave,
                      loading: saving.value,
                      disabled: !form.value.name.trim() || !form.value.cron.trim(),
                      class: "ml-3 px-8"
                    }, {
                      default: withCtx(() => [
                        createVNode(VIcon, {
                          start: "",
                          size: "20"
                        }, {
                          default: withCtx(() => [
                            createTextVNode(toDisplayString(editingCron.value ? "mdi-content-save" : "mdi-plus"), 1)
                          ]),
                          _: 1
                        }),
                        createTextVNode(" " + toDisplayString(editingCron.value ? "Save Changes" : "Create Cron"), 1)
                      ]),
                      _: 1
                    }, 8, ["loading", "disabled"])
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(VDialog, {
          modelValue: deleteDialog.value,
          "onUpdate:modelValue": _cache[20] || (_cache[20] = ($event) => deleteDialog.value = $event),
          "max-width": "440",
          transition: "dialog-bottom-transition"
        }, {
          default: withCtx(() => [
            createVNode(VCard, { class: "dialog-card" }, {
              default: withCtx(() => [
                _cache[71] || (_cache[71] = createBaseVNode("div", { class: "dialog-top-bar bg-error" }, null, -1)),
                createVNode(VCardTitle, { class: "d-flex align-center pt-5" }, {
                  default: withCtx(() => [
                    createVNode(VIcon, {
                      color: "error",
                      size: "24",
                      class: "mr-2"
                    }, {
                      default: withCtx(() => [..._cache[64] || (_cache[64] = [
                        createTextVNode("mdi-delete-alert", -1)
                      ])]),
                      _: 1
                    }),
                    _cache[65] || (_cache[65] = createTextVNode(" Delete Cron Job ", -1))
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      createBaseVNode("p", _hoisted_55, [
                        _cache[66] || (_cache[66] = createTextVNode(" Are you sure you want to delete ", -1)),
                        createBaseVNode("strong", _hoisted_56, toDisplayString((_a = deleteTarget.value) == null ? void 0 : _a.name), 1),
                        _cache[67] || (_cache[67] = createTextVNode("? ", -1))
                      ]),
                      createVNode(VAlert, {
                        type: "warning",
                        variant: "tonal",
                        density: "compact",
                        class: "mt-3"
                      }, {
                        default: withCtx(() => [..._cache[68] || (_cache[68] = [
                          createTextVNode(" This action cannot be undone. All execution history will be permanently removed. ", -1)
                        ])]),
                        _: 1
                      })
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, { class: "px-6 pb-5" }, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "outlined",
                      onClick: _cache[19] || (_cache[19] = ($event) => deleteDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[69] || (_cache[69] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      variant: "elevated",
                      onClick: handleDelete,
                      loading: deleting.value,
                      class: "ml-3"
                    }, {
                      default: withCtx(() => [..._cache[70] || (_cache[70] = [
                        createTextVNode(" Delete ", -1)
                      ])]),
                      _: 1
                    }, 8, ["loading"])
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(VDialog, {
          modelValue: historyDialog.value,
          "onUpdate:modelValue": _cache[22] || (_cache[22] = ($event) => historyDialog.value = $event),
          "max-width": "760",
          transition: "dialog-bottom-transition"
        }, {
          default: withCtx(() => [
            createVNode(VCard, { class: "dialog-card" }, {
              default: withCtx(() => [
                _cache[78] || (_cache[78] = createBaseVNode("div", { class: "dialog-top-bar bg-primary" }, null, -1)),
                createVNode(VCardTitle, { class: "d-flex align-center pt-5" }, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      createVNode(VIcon, {
                        color: "primary",
                        size: "24",
                        class: "mr-2"
                      }, {
                        default: withCtx(() => [..._cache[72] || (_cache[72] = [
                          createTextVNode("mdi-history", -1)
                        ])]),
                        _: 1
                      }),
                      createTextVNode(" Execution History — " + toDisplayString((_a = historyTarget.value) == null ? void 0 : _a.name), 1)
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createBaseVNode("div", _hoisted_57, [
                      createVNode(VBtn, {
                        variant: "tonal",
                        size: "small",
                        "prepend-icon": "mdi-refresh",
                        onClick: refreshHistory,
                        loading: historyLoading.value
                      }, {
                        default: withCtx(() => [..._cache[73] || (_cache[73] = [
                          createTextVNode(" Refresh ", -1)
                        ])]),
                        _: 1
                      }, 8, ["loading"]),
                      createVNode(VSpacer),
                      historyLogs.value.length ? (openBlock(), createElementBlock("span", _hoisted_58, toDisplayString(historyLogs.value.length) + " record" + toDisplayString(historyLogs.value.length > 1 ? "s" : ""), 1)) : createCommentVNode("", true)
                    ]),
                    historyLoading.value ? (openBlock(), createElementBlock("div", _hoisted_59, [
                      createVNode(VProgressCircular, {
                        indeterminate: "",
                        size: "32",
                        width: "3",
                        color: "primary"
                      })
                    ])) : historyLogs.value.length > 0 ? (openBlock(), createBlock(VList, {
                      key: 1,
                      lines: "two",
                      density: "compact",
                      class: "history-list rounded-lg"
                    }, {
                      default: withCtx(() => [
                        (openBlock(true), createElementBlock(Fragment, null, renderList(historyLogs.value, (log) => {
                          return openBlock(), createBlock(VListItem, {
                            key: log.id,
                            rounded: "lg",
                            class: normalizeClass(["mb-1 history-item", log.success ? "history-success" : "history-error"])
                          }, {
                            prepend: withCtx(() => [
                              createVNode(VAvatar, {
                                color: log.success ? "success" : "error",
                                size: "32",
                                variant: "tonal"
                              }, {
                                default: withCtx(() => [
                                  createVNode(VIcon, {
                                    color: log.success ? "success" : "error",
                                    size: "18"
                                  }, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(log.success ? "mdi-check-circle" : "mdi-alert-circle"), 1)
                                    ]),
                                    _: 2
                                  }, 1032, ["color"])
                                ]),
                                _: 2
                              }, 1032, ["color"])
                            ]),
                            default: withCtx(() => [
                              createVNode(VListItemTitle, { class: "d-flex align-center" }, {
                                default: withCtx(() => [
                                  createBaseVNode("span", _hoisted_60, toDisplayString(formatDate(log.executed_at)), 1),
                                  createVNode(VChip, {
                                    color: log.success ? "success" : "error",
                                    size: "x-small",
                                    variant: "tonal",
                                    class: "ml-2",
                                    label: ""
                                  }, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(log.success ? "Success" : "Failed"), 1)
                                    ]),
                                    _: 2
                                  }, 1032, ["color"])
                                ]),
                                _: 2
                              }, 1024),
                              log.message ? (openBlock(), createBlock(VListItemSubtitle, {
                                key: 0,
                                class: "mt-1"
                              }, {
                                default: withCtx(() => [
                                  createBaseVNode("pre", {
                                    class: normalizeClass(["log-output", { "text-error": !log.success }])
                                  }, toDisplayString(log.message), 3)
                                ]),
                                _: 2
                              }, 1024)) : createCommentVNode("", true)
                            ]),
                            _: 2
                          }, 1032, ["class"]);
                        }), 128))
                      ]),
                      _: 1
                    })) : (openBlock(), createElementBlock("div", _hoisted_61, [
                      createVNode(VIcon, {
                        size: "48",
                        color: "medium-emphasis",
                        class: "mb-3"
                      }, {
                        default: withCtx(() => [..._cache[74] || (_cache[74] = [
                          createTextVNode("mdi-text-box-outline", -1)
                        ])]),
                        _: 1
                      }),
                      _cache[75] || (_cache[75] = createBaseVNode("p", { class: "text-body-1 text-medium-emphasis" }, "No execution history yet", -1)),
                      _cache[76] || (_cache[76] = createBaseVNode("p", { class: "text-caption text-medium-emphasis mt-1" }, "Records will appear here after the cron job runs", -1))
                    ]))
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, { class: "px-6 pb-5" }, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "outlined",
                      onClick: _cache[21] || (_cache[21] = ($event) => historyDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[77] || (_cache[77] = [
                        createTextVNode("Close", -1)
                      ])]),
                      _: 1
                    })
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"])
      ]);
    };
  }
});
const CronsPage = /* @__PURE__ */ _export_sfc(_sfc_main, [["__scopeId", "data-v-1aa8bd4f"]]);
export {
  CronsPage as default
};
