import { S as defineStore, a2 as ref, J as api, R as defineComponent, a9 as useAppStore, a0 as onMounted, O as createElementBlock, L as createBaseVNode, Q as createVNode, ag as withCtx, q as VIcon, d as VBtn, y as VRow, a8 as unref, M as createBlock, N as createCommentVNode, n as VDialog, a1 as openBlock, P as createTextVNode, F as Fragment, a3 as renderList, e as VCard, g as VCardItem, j as VCardTitle, a6 as toDisplayString, h as VCardSubtitle, i as VCardText, f as VCardActions, C as VSwitch, B as VSpacer, m as VCol, G as VTextField, z as VSelect, H as VTextarea, A as VSkeletonLoader, _ as _export_sfc } from "./index-CD7sFTTo.js";
const useToolStore = defineStore("tools", () => {
  const tools = ref([]);
  const loading = ref(false);
  async function fetchTools() {
    loading.value = true;
    try {
      tools.value = await api.getTools();
    } catch (e) {
      console.error("Failed to fetch tools:", e);
      tools.value = [];
    } finally {
      loading.value = false;
    }
  }
  async function addTool(data) {
    const tool = await api.createTool(data);
    tools.value.push(tool);
    return tool;
  }
  async function updateTool(id, data) {
    const tool = await api.updateTool(id, data);
    const idx = tools.value.findIndex((t) => t.id === id);
    if (idx >= 0) {
      tools.value[idx] = tool;
    }
    return tool;
  }
  async function removeTool(id) {
    await api.deleteTool(id);
    tools.value = tools.value.filter((t) => t.id !== id);
  }
  return {
    tools,
    loading,
    fetchTools,
    addTool,
    updateTool,
    removeTool
  };
});
const _hoisted_1 = { class: "d-flex align-center justify-space-between mb-6" };
const _hoisted_2 = { class: "d-flex align-center" };
const _hoisted_3 = {
  key: 0,
  class: "text-body-2 mb-3 description-preview"
};
const _hoisted_4 = {
  key: 1,
  class: "mb-3"
};
const _hoisted_5 = { class: "config-preview text-caption" };
const _hoisted_6 = {
  key: 1,
  class: "text-center py-12"
};
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "ToolsPage",
  setup(__props) {
    const toolStore = useToolStore();
    const appStore = useAppStore();
    const showDialog = ref(false);
    const editingTool = ref(null);
    const saving = ref(false);
    const deleteDialog = ref(false);
    const deleteTarget = ref(null);
    const deleting = ref(false);
    const availableTypes = [
      "function",
      "browser",
      "search",
      "file",
      "custom"
    ];
    const defaultForm = () => ({
      name: "",
      description: "",
      type: "function",
      config: "",
      enabled: true
    });
    const form = ref(defaultForm());
    function openAddDialog() {
      editingTool.value = null;
      form.value = defaultForm();
      showDialog.value = true;
    }
    function openEditDialog(tool) {
      editingTool.value = tool;
      form.value = {
        name: tool.name,
        description: tool.description,
        type: tool.type,
        config: tool.config,
        enabled: tool.enabled
      };
      showDialog.value = true;
    }
    async function handleSave() {
      saving.value = true;
      try {
        if (editingTool.value) {
          await toolStore.updateTool(editingTool.value.id, { ...form.value });
          appStore.showMessage("Tool updated", "success");
        } else {
          await toolStore.addTool({ ...form.value });
          appStore.showMessage("Tool created", "success");
        }
        showDialog.value = false;
        form.value = defaultForm();
        editingTool.value = null;
      } catch {
        appStore.showMessage("Failed to save tool", "error");
      } finally {
        saving.value = false;
      }
    }
    function confirmRemove(tool) {
      deleteTarget.value = tool;
      deleteDialog.value = true;
    }
    async function handleDelete() {
      if (!deleteTarget.value) return;
      deleting.value = true;
      try {
        await toolStore.removeTool(deleteTarget.value.id);
        appStore.showMessage("Tool deleted", "success");
      } catch {
        appStore.showMessage("Failed to delete tool", "error");
      } finally {
        deleting.value = false;
        deleteDialog.value = false;
        deleteTarget.value = null;
      }
    }
    async function handleToggleEnabled(tool, enabled) {
      try {
        await toolStore.updateTool(tool.id, { enabled });
        appStore.showMessage(enabled ? "Tool enabled" : "Tool disabled", "success");
      } catch {
        appStore.showMessage("Failed to update tool", "error");
      }
    }
    onMounted(() => {
      toolStore.fetchTools();
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
              default: withCtx(() => [..._cache[9] || (_cache[9] = [
                createTextVNode("mdi-hammer-wrench", -1)
              ])]),
              _: 1
            }),
            _cache[10] || (_cache[10] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "Tools", -1))
          ]),
          createVNode(VBtn, {
            color: "primary",
            "prepend-icon": "mdi-plus",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[11] || (_cache[11] = [
              createTextVNode(" Add Tool ", -1)
            ])]),
            _: 1
          })
        ]),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(toolStore).tools, (tool) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                md: "6",
                lg: "4",
                key: tool.id
              }, {
                default: withCtx(() => [
                  createVNode(VCard, { class: "fill-height" }, {
                    default: withCtx(() => [
                      createVNode(VCardItem, null, {
                        prepend: withCtx(() => [
                          createVNode(VIcon, {
                            color: tool.enabled ? "primary" : "medium-emphasis",
                            size: "28"
                          }, {
                            default: withCtx(() => [..._cache[12] || (_cache[12] = [
                              createTextVNode(" mdi-hammer-wrench ", -1)
                            ])]),
                            _: 1
                          }, 8, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(tool.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          createVNode(VCardSubtitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(tool.type), 1)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, null, {
                        default: withCtx(() => [
                          tool.description ? (openBlock(), createElementBlock("div", _hoisted_3, toDisplayString(tool.description), 1)) : createCommentVNode("", true),
                          tool.config ? (openBlock(), createElementBlock("div", _hoisted_4, [
                            _cache[13] || (_cache[13] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Config", -1)),
                            createBaseVNode("div", _hoisted_5, toDisplayString(tool.config), 1)
                          ])) : createCommentVNode("", true)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, null, {
                        default: withCtx(() => [
                          createVNode(VSwitch, {
                            "model-value": tool.enabled,
                            label: tool.enabled ? "Enabled" : "Disabled",
                            color: "success",
                            density: "compact",
                            "hide-details": "",
                            "onUpdate:modelValue": (v) => handleToggleEnabled(tool, !!v)
                          }, null, 8, ["model-value", "label", "onUpdate:modelValue"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-pencil",
                            onClick: ($event) => openEditDialog(tool)
                          }, {
                            default: withCtx(() => [..._cache[14] || (_cache[14] = [
                              createTextVNode(" Edit ", -1)
                            ])]),
                            _: 1
                          }, 8, ["onClick"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            icon: "mdi-delete",
                            variant: "text",
                            size: "small",
                            color: "error",
                            onClick: ($event) => confirmRemove(tool)
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
        unref(toolStore).loading ? (openBlock(), createBlock(VSkeletonLoader, {
          key: 0,
          type: "card@3"
        })) : createCommentVNode("", true),
        !unref(toolStore).loading && unref(toolStore).tools.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_6, [
          createVNode(VIcon, {
            size: "64",
            color: "medium-emphasis"
          }, {
            default: withCtx(() => [..._cache[15] || (_cache[15] = [
              createTextVNode("mdi-hammer-wrench", -1)
            ])]),
            _: 1
          }),
          _cache[17] || (_cache[17] = createBaseVNode("p", { class: "text-medium-emphasis mt-4" }, "No tools configured", -1)),
          createVNode(VBtn, {
            color: "primary",
            class: "mt-4",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[16] || (_cache[16] = [
              createTextVNode("Add Tool", -1)
            ])]),
            _: 1
          })
        ])) : createCommentVNode("", true),
        createVNode(VDialog, {
          modelValue: showDialog.value,
          "onUpdate:modelValue": _cache[6] || (_cache[6] = ($event) => showDialog.value = $event),
          "max-width": "600"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(editingTool.value ? "Edit Tool" : "Add Tool"), 1)
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
                              placeholder: "file-reader",
                              hint: "Unique name for the tool",
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
                            createVNode(VSelect, {
                              modelValue: form.value.type,
                              "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => form.value.type = $event),
                              label: "Type",
                              items: availableTypes,
                              variant: "outlined"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextarea, {
                              modelValue: form.value.description,
                              "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => form.value.description = $event),
                              label: "Description",
                              variant: "outlined",
                              rows: "2",
                              placeholder: "Tool description..."
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextarea, {
                              modelValue: form.value.config,
                              "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => form.value.config = $event),
                              label: "Config",
                              variant: "outlined",
                              rows: "6",
                              placeholder: '{"type": "function", "function": {...}}'
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
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
                        })
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
                      onClick: _cache[5] || (_cache[5] = ($event) => showDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[18] || (_cache[18] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      onClick: handleSave,
                      loading: saving.value,
                      disabled: !form.value.name.trim()
                    }, {
                      default: withCtx(() => [
                        createTextVNode(toDisplayString(editingTool.value ? "Update" : "Create"), 1)
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
          "onUpdate:modelValue": _cache[8] || (_cache[8] = ($event) => deleteDialog.value = $event),
          "max-width": "400"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [..._cache[19] || (_cache[19] = [
                    createTextVNode("Delete Tool", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      createBaseVNode("p", null, [
                        _cache[20] || (_cache[20] = createTextVNode(" Are you sure you want to delete ", -1)),
                        createBaseVNode("strong", null, toDisplayString((_a = deleteTarget.value) == null ? void 0 : _a.name), 1),
                        _cache[21] || (_cache[21] = createTextVNode("? ", -1))
                      ]),
                      _cache[22] || (_cache[22] = createBaseVNode("p", { class: "text-caption text-error mt-2" }, "This action cannot be undone.", -1))
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[7] || (_cache[7] = ($event) => deleteDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[23] || (_cache[23] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      onClick: handleDelete,
                      loading: deleting.value
                    }, {
                      default: withCtx(() => [..._cache[24] || (_cache[24] = [
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
        }, 8, ["modelValue"])
      ]);
    };
  }
});
const ToolsPage = /* @__PURE__ */ _export_sfc(_sfc_main, [["__scopeId", "data-v-bc5265b8"]]);
export {
  ToolsPage as default
};
