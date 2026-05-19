import { S as defineStore, a2 as ref, J as api, R as defineComponent, a9 as useAppStore, a0 as onMounted, O as createElementBlock, L as createBaseVNode, Q as createVNode, ag as withCtx, q as VIcon, d as VBtn, y as VRow, a8 as unref, M as createBlock, N as createCommentVNode, n as VDialog, a1 as openBlock, P as createTextVNode, F as Fragment, a3 as renderList, e as VCard, g as VCardItem, j as VCardTitle, a6 as toDisplayString, h as VCardSubtitle, i as VCardText, m as VCol, f as VCardActions, C as VSwitch, B as VSpacer, aj as withModifiers, G as VTextField, H as VTextarea, o as VDivider, V as VAlert, l as VChip, x as VProgressCircular, t as VListItem, v as VListItemTitle, u as VListItemSubtitle, Z as normalizeClass, r as VList, A as VSkeletonLoader, _ as _export_sfc } from "./index-CD7sFTTo.js";
const useCronStore = defineStore("crons", () => {
  const crons = ref([]);
  const loading = ref(false);
  async function fetchCrons() {
    loading.value = true;
    try {
      crons.value = await api.getCrons();
    } catch (e) {
      console.error("Failed to fetch crons:", e);
      crons.value = [];
    } finally {
      loading.value = false;
    }
  }
  async function addCron(data) {
    const cron = await api.createCron(data);
    crons.value.push(cron);
    return cron;
  }
  async function updateCron(id, data) {
    const cron = await api.updateCron(id, data);
    const idx = crons.value.findIndex((c) => c.id === id);
    if (idx >= 0) {
      crons.value[idx] = cron;
    }
    return cron;
  }
  async function removeCron(id) {
    await api.deleteCron(id);
    crons.value = crons.value.filter((c) => c.id !== id);
  }
  async function fetchCronLogs(jobId) {
    return api.getCronLogs(jobId);
  }
  return {
    crons,
    loading,
    fetchCrons,
    addCron,
    updateCron,
    removeCron,
    fetchCronLogs
  };
});
const _hoisted_1 = { class: "d-flex align-center justify-space-between mb-6" };
const _hoisted_2 = { class: "d-flex align-center" };
const _hoisted_3 = { class: "text-body-2 cron-expr" };
const _hoisted_4 = { class: "text-body-2" };
const _hoisted_5 = { class: "text-body-2" };
const _hoisted_6 = { class: "text-body-2 text-truncate" };
const _hoisted_7 = {
  key: 1,
  class: "text-center py-12"
};
const _hoisted_8 = { class: "d-flex align-center mb-3" };
const _hoisted_9 = {
  key: 0,
  class: "text-center pa-4"
};
const _hoisted_10 = {
  key: 2,
  class: "text-center py-6"
};
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "CronsPage",
  setup(__props) {
    const cronStore = useCronStore();
    const appStore = useAppStore();
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
    const cronPresets = [
      { label: "Every minute", expr: "0 * * * * *" },
      { label: "Every 5 min", expr: "0 */5 * * * *" },
      { label: "Every 15 min", expr: "0 */15 * * * *" },
      { label: "Every hour", expr: "0 0 * * * *" },
      { label: "Daily midnight (UTC)", expr: "0 0 0 * * *" },
      { label: "Daily 6 AM (UTC)", expr: "0 0 6 * * *" },
      { label: "Weekends", expr: "0 0 6 * * Sun,Sat" },
      { label: "Monthly 1st", expr: "0 0 0 1 * *" },
      { label: "Weekdays 9AM (UTC)", expr: "0 0 9 * * Mon-Fri" }
    ];
    const defaultForm = () => ({
      name: "",
      description: "",
      cron: "",
      enabled: true,
      timeout: null,
      prompt: null
    });
    const form = ref(defaultForm());
    function openAddDialog() {
      editingCron.value = null;
      form.value = defaultForm();
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
        prompt: cron.prompt
      };
      showDialog.value = true;
    }
    async function handleSave() {
      saving.value = true;
      try {
        if (editingCron.value) {
          await cronStore.updateCron(editingCron.value.id, { ...form.value });
          appStore.showMessage("Cron updated", "success");
        } else {
          await cronStore.addCron({ ...form.value });
          appStore.showMessage("Cron created", "success");
        }
        showDialog.value = false;
        form.value = defaultForm();
        editingCron.value = null;
        await cronStore.fetchCrons();
      } catch {
        appStore.showMessage("Failed to save cron", "error");
      } finally {
        saving.value = false;
      }
    }
    function confirmRemove(cron) {
      deleteTarget.value = cron;
      deleteDialog.value = true;
    }
    async function handleDelete() {
      if (!deleteTarget.value) return;
      deleting.value = true;
      try {
        await cronStore.removeCron(deleteTarget.value.id);
        appStore.showMessage("Cron deleted", "success");
        await cronStore.fetchCrons();
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
        await cronStore.fetchCrons();
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
      return new Date(dateStr).toLocaleDateString("zh-CN", { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" });
    }
    onMounted(() => {
      cronStore.fetchCrons();
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createBaseVNode("div", _hoisted_2, [
            createVNode(VIcon, {
              size: "32",
              color: "primary",
              class: "mr-3"
            }, {
              default: withCtx(() => [..._cache[12] || (_cache[12] = [
                createTextVNode("mdi-clock-outline", -1)
              ])]),
              _: 1
            }),
            _cache[13] || (_cache[13] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "Cron Jobs", -1))
          ]),
          createVNode(VBtn, {
            color: "primary",
            "prepend-icon": "mdi-plus",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[14] || (_cache[14] = [
              createTextVNode(" Add Cron ", -1)
            ])]),
            _: 1
          })
        ]),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(cronStore).crons, (cron) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                key: cron.id
              }, {
                default: withCtx(() => [
                  createVNode(VCard, null, {
                    default: withCtx(() => [
                      createVNode(VCardItem, null, {
                        prepend: withCtx(() => [
                          createVNode(VIcon, {
                            color: cron.enabled ? "success" : "medium-emphasis",
                            size: "28"
                          }, {
                            default: withCtx(() => [..._cache[15] || (_cache[15] = [
                              createTextVNode(" mdi-clock-outline ", -1)
                            ])]),
                            _: 1
                          }, 8, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(cron.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          cron.description ? (openBlock(), createBlock(VCardSubtitle, { key: 0 }, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(cron.description), 1)
                            ]),
                            _: 2
                          }, 1024)) : createCommentVNode("", true)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, null, {
                        default: withCtx(() => [
                          createVNode(VRow, { dense: "" }, {
                            default: withCtx(() => [
                              createVNode(VCol, {
                                cols: "12",
                                md: "6"
                              }, {
                                default: withCtx(() => [
                                  _cache[16] || (_cache[16] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Cron Expression", -1)),
                                  createBaseVNode("code", _hoisted_3, toDisplayString(cron.cron), 1)
                                ]),
                                _: 2
                              }, 1024),
                              createVNode(VCol, {
                                cols: "6",
                                md: "3"
                              }, {
                                default: withCtx(() => [
                                  _cache[17] || (_cache[17] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Timeout", -1)),
                                  createBaseVNode("div", _hoisted_4, toDisplayString(cron.timeout ? `${cron.timeout}s` : "-"), 1)
                                ]),
                                _: 2
                              }, 1024),
                              createVNode(VCol, {
                                cols: "6",
                                md: "3"
                              }, {
                                default: withCtx(() => [
                                  _cache[18] || (_cache[18] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Last Run", -1)),
                                  createBaseVNode("div", _hoisted_5, toDisplayString(cron.last_run_at ? formatDate(cron.last_run_at) : "Never"), 1)
                                ]),
                                _: 2
                              }, 1024)
                            ]),
                            _: 2
                          }, 1024),
                          createVNode(VRow, {
                            dense: "",
                            class: "mt-2"
                          }, {
                            default: withCtx(() => [
                              createVNode(VCol, { cols: "12" }, {
                                default: withCtx(() => [
                                  _cache[19] || (_cache[19] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Prompt", -1)),
                                  createBaseVNode("div", _hoisted_6, toDisplayString(cron.prompt || "-"), 1)
                                ]),
                                _: 2
                              }, 1024)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, null, {
                        default: withCtx(() => [
                          createVNode(VSwitch, {
                            "model-value": cron.enabled,
                            label: cron.enabled ? "Enabled" : "Disabled",
                            color: "success",
                            density: "compact",
                            "hide-details": "",
                            "onUpdate:modelValue": (v) => handleToggleEnabled(cron, !!v)
                          }, null, 8, ["model-value", "label", "onUpdate:modelValue"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-history",
                            onClick: withModifiers(($event) => openHistory(cron), ["stop"])
                          }, {
                            default: withCtx(() => [..._cache[20] || (_cache[20] = [
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
                            default: withCtx(() => [..._cache[21] || (_cache[21] = [
                              createTextVNode(" Edit ", -1)
                            ])]),
                            _: 1
                          }, 8, ["onClick"]),
                          createVNode(VBtn, {
                            icon: "mdi-delete",
                            variant: "text",
                            size: "small",
                            color: "error",
                            onClick: ($event) => confirmRemove(cron)
                          }, null, 8, ["onClick"])
                        ]),
                        _: 2
                      }, 1024)
                    ]),
                    _: 2
                  }, 1024)
                ]),
                _: 2
              }, 1024);
            }), 128))
          ]),
          _: 1
        }),
        unref(cronStore).loading ? (openBlock(), createBlock(VSkeletonLoader, {
          key: 0,
          type: "card@3"
        })) : createCommentVNode("", true),
        !unref(cronStore).loading && unref(cronStore).crons.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_7, [
          createVNode(VIcon, {
            size: "64",
            color: "medium-emphasis"
          }, {
            default: withCtx(() => [..._cache[22] || (_cache[22] = [
              createTextVNode("mdi-clock-off", -1)
            ])]),
            _: 1
          }),
          _cache[24] || (_cache[24] = createBaseVNode("p", { class: "text-medium-emphasis mt-4" }, "No cron jobs configured", -1)),
          createVNode(VBtn, {
            color: "primary",
            class: "mt-4",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[23] || (_cache[23] = [
              createTextVNode("Add Cron", -1)
            ])]),
            _: 1
          })
        ])) : createCommentVNode("", true),
        createVNode(VDialog, {
          modelValue: showDialog.value,
          "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => showDialog.value = $event),
          "max-width": "600"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(editingCron.value ? "Edit Cron" : "Add Cron"), 1)
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createVNode(VRow, { dense: "" }, {
                      default: withCtx(() => [
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.name,
                              "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => form.value.name = $event),
                              label: "Name",
                              variant: "outlined",
                              placeholder: "daily-backup",
                              hint: "Unique name for the cron job",
                              "persistent-hint": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextarea, {
                              modelValue: form.value.description,
                              "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => form.value.description = $event),
                              label: "Description",
                              variant: "outlined",
                              rows: "2",
                              placeholder: "Cron job description..."
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.cron,
                              "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => form.value.cron = $event),
                              label: "Cron Expression",
                              variant: "outlined",
                              placeholder: "0 0 0 * * *",
                              hint: "sec min hour day month day_of_week (UTC timezone)",
                              "persistent-hint": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.timeout,
                              "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => form.value.timeout = $event),
                              modelModifiers: { number: true },
                              label: "Timeout",
                              type: "number",
                              variant: "outlined",
                              placeholder: "30",
                              hint: "Timeout in seconds",
                              "persistent-hint": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, {
                          cols: "12",
                          md: "6"
                        }, {
                          default: withCtx(() => [
                            createVNode(VSwitch, {
                              modelValue: form.value.enabled,
                              "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => form.value.enabled = $event),
                              label: "Enabled",
                              color: "success",
                              "hide-details": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextarea, {
                              modelValue: form.value.prompt,
                              "onUpdate:modelValue": _cache[5] || (_cache[5] = ($event) => form.value.prompt = $event),
                              label: "Prompt",
                              variant: "outlined",
                              rows: "4",
                              placeholder: "Enter your prompt here..."
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        })
                      ]),
                      _: 1
                    }),
                    createVNode(VDivider, { class: "my-4" }),
                    _cache[26] || (_cache[26] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-2" }, [
                      createTextVNode(" Cron Expression: "),
                      createBaseVNode("code", { class: "ml-2" }, "sec min hour day month day_of_week"),
                      createTextVNode(" (UTC timezone) ")
                    ], -1)),
                    createVNode(VAlert, {
                      type: "info",
                      variant: "tonal",
                      density: "compact",
                      class: "mb-3"
                    }, {
                      default: withCtx(() => [..._cache[25] || (_cache[25] = [
                        createBaseVNode("ul", {
                          class: "mb-0",
                          style: { "list-style-type": "disc", "padding-left": "1.5rem", "font-size": "0.75rem" }
                        }, [
                          createBaseVNode("li", null, [
                            createTextVNode("Comma-separated: "),
                            createBaseVNode("code", null, "0 2,14,26 * * * *")
                          ]),
                          createBaseVNode("li", null, [
                            createTextVNode("Ranges: "),
                            createBaseVNode("code", null, "0 0 * 5-10 * *")
                          ]),
                          createBaseVNode("li", null, [
                            createTextVNode("Day of week: "),
                            createBaseVNode("code", null, "0 0 6 * * Sun,Sat")
                          ])
                        ], -1)
                      ])]),
                      _: 1
                    }),
                    _cache[27] || (_cache[27] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-2" }, "Quick Presets", -1)),
                    createVNode(VRow, { dense: "" }, {
                      default: withCtx(() => [
                        (openBlock(), createElementBlock(Fragment, null, renderList(cronPresets, (preset) => {
                          return createVNode(VCol, {
                            cols: "4",
                            key: preset.label
                          }, {
                            default: withCtx(() => [
                              createVNode(VChip, {
                                size: "x-small",
                                variant: "tonal",
                                class: "mr-1 mb-1",
                                onClick: ($event) => form.value.cron = preset.expr
                              }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(preset.label), 1)
                                ]),
                                _: 2
                              }, 1032, ["onClick"])
                            ]),
                            _: 2
                          }, 1024);
                        }), 64))
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[6] || (_cache[6] = ($event) => showDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[28] || (_cache[28] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      onClick: handleSave,
                      loading: saving.value,
                      disabled: !form.value.name.trim() || !form.value.cron.trim()
                    }, {
                      default: withCtx(() => [
                        createTextVNode(toDisplayString(editingCron.value ? "Update" : "Create"), 1)
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
          "onUpdate:modelValue": _cache[9] || (_cache[9] = ($event) => deleteDialog.value = $event),
          "max-width": "400"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [..._cache[29] || (_cache[29] = [
                    createTextVNode("Delete Cron", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      createBaseVNode("p", null, [
                        _cache[30] || (_cache[30] = createTextVNode(" Are you sure you want to delete ", -1)),
                        createBaseVNode("strong", null, toDisplayString((_a = deleteTarget.value) == null ? void 0 : _a.name), 1),
                        _cache[31] || (_cache[31] = createTextVNode("? ", -1))
                      ]),
                      _cache[32] || (_cache[32] = createBaseVNode("p", { class: "text-caption text-error mt-2" }, "This action cannot be undone.", -1))
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[8] || (_cache[8] = ($event) => deleteDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[33] || (_cache[33] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      onClick: handleDelete,
                      loading: deleting.value
                    }, {
                      default: withCtx(() => [..._cache[34] || (_cache[34] = [
                        createTextVNode("Delete", -1)
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
          "onUpdate:modelValue": _cache[11] || (_cache[11] = ($event) => historyDialog.value = $event),
          "max-width": "700"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, { class: "d-flex align-center" }, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      createVNode(VIcon, { start: "" }, {
                        default: withCtx(() => [..._cache[35] || (_cache[35] = [
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
                    createBaseVNode("div", _hoisted_8, [
                      createVNode(VBtn, {
                        variant: "tonal",
                        size: "small",
                        "prepend-icon": "mdi-refresh",
                        onClick: refreshHistory,
                        loading: historyLoading.value
                      }, {
                        default: withCtx(() => [..._cache[36] || (_cache[36] = [
                          createTextVNode(" Refresh ", -1)
                        ])]),
                        _: 1
                      }, 8, ["loading"])
                    ]),
                    historyLoading.value ? (openBlock(), createElementBlock("div", _hoisted_9, [
                      createVNode(VProgressCircular, {
                        indeterminate: "",
                        size: "24",
                        width: "2",
                        color: "primary"
                      })
                    ])) : historyLogs.value.length > 0 ? (openBlock(), createBlock(VList, {
                      key: 1,
                      lines: "two",
                      density: "compact"
                    }, {
                      default: withCtx(() => [
                        (openBlock(true), createElementBlock(Fragment, null, renderList(historyLogs.value, (log) => {
                          return openBlock(), createBlock(VListItem, {
                            key: log.id,
                            rounded: "lg",
                            class: "mb-1"
                          }, {
                            prepend: withCtx(() => [
                              createVNode(VIcon, {
                                color: log.success ? "success" : "error",
                                size: "20"
                              }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(log.success ? "mdi-check-circle" : "mdi-alert-circle"), 1)
                                ]),
                                _: 2
                              }, 1032, ["color"])
                            ]),
                            default: withCtx(() => [
                              createVNode(VListItemTitle, { class: "text-body-2" }, {
                                default: withCtx(() => [
                                  createTextVNode(toDisplayString(formatDate(log.executed_at)) + " ", 1),
                                  createVNode(VChip, {
                                    color: log.success ? "success" : "error",
                                    size: "x-small",
                                    variant: "tonal",
                                    class: "ml-2"
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
                                    class: normalizeClass(["log-output", !log.success ? "text-error" : ""])
                                  }, toDisplayString(log.message), 3)
                                ]),
                                _: 2
                              }, 1024)) : createCommentVNode("", true)
                            ]),
                            _: 2
                          }, 1024);
                        }), 128))
                      ]),
                      _: 1
                    })) : (openBlock(), createElementBlock("div", _hoisted_10, [
                      createVNode(VIcon, {
                        size: "40",
                        color: "medium-emphasis"
                      }, {
                        default: withCtx(() => [..._cache[37] || (_cache[37] = [
                          createTextVNode("mdi-text-box-outline", -1)
                        ])]),
                        _: 1
                      }),
                      _cache[38] || (_cache[38] = createBaseVNode("p", { class: "text-medium-emphasis mt-2 text-body-2" }, "No execution history yet", -1))
                    ]))
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[10] || (_cache[10] = ($event) => historyDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[39] || (_cache[39] = [
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
const CronsPage = /* @__PURE__ */ _export_sfc(_sfc_main, [["__scopeId", "data-v-f34baefc"]]);
export {
  CronsPage as default
};
