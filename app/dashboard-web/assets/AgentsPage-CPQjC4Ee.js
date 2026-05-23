import { T as defineStore, K as api, S as defineComponent, ab as useAppStore, a1 as onMounted, P as createElementBlock, M as createBaseVNode, R as createVNode, ai as withCtx, r as VIcon, e as VBtn, z as VRow, aa as unref, N as createBlock, O as createCommentVNode, o as VDialog, a4 as ref, a2 as openBlock, Q as createTextVNode, F as Fragment, a5 as renderList, f as VCard, h as VCardItem, k as VCardTitle, a8 as toDisplayString, i as VCardSubtitle, j as VCardText, m as VChip, g as VCardActions, D as VSwitch, C as VSpacer, n as VCol, H as VTextField, A as VSelect, I as VTextarea, B as VSkeletonLoader, _ as _export_sfc } from "./main-gWZPyuWK.js";
import { u as useCrudOperations } from "./baseCrud-wwuZycIH.js";
const useAgentStore = defineStore("agents", () => {
  const crud = useCrudOperations(
    {
      fetchAll: () => api.getAgents(),
      create: (data) => api.createAgent(data),
      update: (id, data) => api.updateAgent(id, data),
      remove: (id) => api.deleteAgent(id)
    },
    "agents"
  );
  return {
    agents: crud.items,
    loading: crud.loading,
    fetchAgents: crud.fetch,
    addAgent: crud.add,
    updateAgent: crud.update,
    removeAgent: crud.remove
  };
});
const _hoisted_1 = { class: "d-flex align-center justify-space-between mb-6" };
const _hoisted_2 = { class: "d-flex align-center" };
const _hoisted_3 = {
  key: 0,
  class: "text-body-2 mb-3"
};
const _hoisted_4 = {
  key: 1,
  class: "mb-3"
};
const _hoisted_5 = { class: "system-prompt-preview text-caption" };
const _hoisted_6 = {
  key: 2,
  class: "mb-2"
};
const _hoisted_7 = { class: "d-flex flex-wrap gap-1" };
const _hoisted_8 = { class: "d-flex align-center gap-2 mt-2" };
const _hoisted_9 = {
  key: 1,
  class: "text-center py-12"
};
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "AgentsPage",
  setup(__props) {
    const agentStore = useAgentStore();
    const appStore = useAppStore();
    const showDialog = ref(false);
    const editingAgent = ref(null);
    const saving = ref(false);
    const deleteDialog = ref(false);
    const deleteTarget = ref(null);
    const deleting = ref(false);
    const availableModels = [
      "claude-3-5-sonnet-20241022",
      "claude-3-opus-20240229",
      "claude-3-sonnet-20240229",
      "claude-3-haiku-20240307",
      "gpt-4o",
      "gpt-4-turbo"
    ];
    const defaultForm = () => ({
      name: "",
      desc: "",
      model: "claude-3-5-sonnet-20241022",
      system_prompt: "",
      tools: "",
      enabled: true
    });
    const form = ref(defaultForm());
    function openAddDialog() {
      editingAgent.value = null;
      form.value = defaultForm();
      showDialog.value = true;
    }
    function openEditDialog(agent) {
      editingAgent.value = agent;
      form.value = {
        name: agent.name,
        desc: agent.desc,
        model: agent.model,
        system_prompt: agent.system_prompt,
        tools: agent.tools,
        enabled: agent.enabled
      };
      showDialog.value = true;
    }
    async function handleSave() {
      saving.value = true;
      try {
        if (editingAgent.value) {
          await agentStore.updateAgent(editingAgent.value.id, { ...form.value });
          appStore.showMessage("Agent updated", "success");
        } else {
          await agentStore.addAgent({ ...form.value });
          appStore.showMessage("Agent created", "success");
        }
        showDialog.value = false;
        form.value = defaultForm();
        editingAgent.value = null;
      } catch {
        appStore.showMessage("Failed to save agent", "error");
      } finally {
        saving.value = false;
      }
    }
    function confirmRemove(agent) {
      deleteTarget.value = agent;
      deleteDialog.value = true;
    }
    async function handleDelete() {
      if (!deleteTarget.value) return;
      deleting.value = true;
      try {
        await agentStore.removeAgent(deleteTarget.value.id);
        appStore.showMessage("Agent deleted", "success");
      } catch {
        appStore.showMessage("Failed to delete agent", "error");
      } finally {
        deleting.value = false;
        deleteDialog.value = false;
        deleteTarget.value = null;
      }
    }
    async function handleToggleEnabled(agent, enabled) {
      try {
        await agentStore.updateAgent(agent.id, { enabled });
        appStore.showMessage(enabled ? "Agent enabled" : "Agent disabled", "success");
      } catch {
        appStore.showMessage("Failed to update agent", "error");
      }
    }
    onMounted(() => {
      agentStore.fetchAgents();
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
              default: withCtx(() => [..._cache[10] || (_cache[10] = [
                createTextVNode("mdi-robot", -1)
              ])]),
              _: 1
            }),
            _cache[11] || (_cache[11] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "Agents", -1))
          ]),
          createVNode(VBtn, {
            color: "primary",
            "prepend-icon": "mdi-plus",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[12] || (_cache[12] = [
              createTextVNode(" Add Agent ", -1)
            ])]),
            _: 1
          })
        ]),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(agentStore).agents, (agent) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                md: "6",
                lg: "4",
                key: agent.id
              }, {
                default: withCtx(() => [
                  createVNode(VCard, { class: "fill-height" }, {
                    default: withCtx(() => [
                      createVNode(VCardItem, null, {
                        prepend: withCtx(() => [
                          createVNode(VIcon, {
                            color: agent.enabled ? "primary" : "medium-emphasis",
                            size: "28"
                          }, {
                            default: withCtx(() => [..._cache[13] || (_cache[13] = [
                              createTextVNode(" mdi-robot ", -1)
                            ])]),
                            _: 1
                          }, 8, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(agent.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          createVNode(VCardSubtitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(agent.model), 1)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, null, {
                        default: withCtx(() => [
                          agent.desc ? (openBlock(), createElementBlock("div", _hoisted_3, toDisplayString(agent.desc), 1)) : createCommentVNode("", true),
                          agent.system_prompt ? (openBlock(), createElementBlock("div", _hoisted_4, [
                            _cache[14] || (_cache[14] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "System Prompt", -1)),
                            createBaseVNode("div", _hoisted_5, toDisplayString(agent.system_prompt), 1)
                          ])) : createCommentVNode("", true),
                          agent.tools ? (openBlock(), createElementBlock("div", _hoisted_6, [
                            _cache[15] || (_cache[15] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Tools", -1)),
                            createBaseVNode("div", _hoisted_7, [
                              (openBlock(true), createElementBlock(Fragment, null, renderList(agent.tools.split(",").map((t) => t.trim()).filter(Boolean), (tool) => {
                                return openBlock(), createBlock(VChip, {
                                  key: tool,
                                  size: "x-small",
                                  variant: "tonal"
                                }, {
                                  default: withCtx(() => [
                                    createTextVNode(toDisplayString(tool), 1)
                                  ]),
                                  _: 2
                                }, 1024);
                              }), 128))
                            ])
                          ])) : createCommentVNode("", true),
                          createBaseVNode("div", _hoisted_8, [
                            createVNode(VChip, {
                              color: agent.enabled ? "success" : "medium-emphasis",
                              size: "x-small",
                              variant: "tonal"
                            }, {
                              default: withCtx(() => [
                                createTextVNode(toDisplayString(agent.enabled ? "Active" : "Inactive"), 1)
                              ]),
                              _: 2
                            }, 1032, ["color"])
                          ])
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, null, {
                        default: withCtx(() => [
                          createVNode(VSwitch, {
                            "model-value": agent.enabled,
                            label: agent.enabled ? "Enabled" : "Disabled",
                            color: "success",
                            density: "compact",
                            "hide-details": "",
                            "onUpdate:modelValue": (v) => handleToggleEnabled(agent, !!v)
                          }, null, 8, ["model-value", "label", "onUpdate:modelValue"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-pencil",
                            onClick: ($event) => openEditDialog(agent)
                          }, {
                            default: withCtx(() => [..._cache[16] || (_cache[16] = [
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
                            onClick: ($event) => confirmRemove(agent)
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
        unref(agentStore).loading ? (openBlock(), createBlock(VSkeletonLoader, {
          key: 0,
          type: "card@3"
        })) : createCommentVNode("", true),
        !unref(agentStore).loading && unref(agentStore).agents.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_9, [
          createVNode(VIcon, {
            size: "64",
            color: "medium-emphasis"
          }, {
            default: withCtx(() => [..._cache[17] || (_cache[17] = [
              createTextVNode("mdi-robot-off", -1)
            ])]),
            _: 1
          }),
          _cache[19] || (_cache[19] = createBaseVNode("p", { class: "text-medium-emphasis mt-4" }, "No agents configured", -1)),
          createVNode(VBtn, {
            color: "primary",
            class: "mt-4",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[18] || (_cache[18] = [
              createTextVNode("Add Agent", -1)
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
                    createTextVNode(toDisplayString(editingAgent.value ? "Edit Agent" : "Add Agent"), 1)
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
                              placeholder: "code-reviewer",
                              hint: "Unique name for the agent",
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
                              modelValue: form.value.model,
                              "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => form.value.model = $event),
                              label: "Model",
                              items: availableModels,
                              variant: "outlined"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextarea, {
                              modelValue: form.value.desc,
                              "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => form.value.desc = $event),
                              label: "Description",
                              variant: "outlined",
                              rows: "2",
                              placeholder: "Agent description..."
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextarea, {
                              modelValue: form.value.system_prompt,
                              "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => form.value.system_prompt = $event),
                              label: "System Prompt",
                              variant: "outlined",
                              rows: "4",
                              placeholder: "You are a helpful assistant..."
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.tools,
                              "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => form.value.tools = $event),
                              label: "Tools",
                              variant: "outlined",
                              placeholder: "Read, Write, Bash, Grep, WebSearch",
                              hint: "Comma-separated list of tool names",
                              "persistent-hint": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VSwitch, {
                              modelValue: form.value.enabled,
                              "onUpdate:modelValue": _cache[5] || (_cache[5] = ($event) => form.value.enabled = $event),
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
                      onClick: _cache[6] || (_cache[6] = ($event) => showDialog.value = false)
                    }, {
                      default: withCtx(() => [..._cache[20] || (_cache[20] = [
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
                        createTextVNode(toDisplayString(editingAgent.value ? "Update" : "Create"), 1)
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
                  default: withCtx(() => [..._cache[21] || (_cache[21] = [
                    createTextVNode("Delete Agent", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      createBaseVNode("p", null, [
                        _cache[22] || (_cache[22] = createTextVNode(" Are you sure you want to delete ", -1)),
                        createBaseVNode("strong", null, toDisplayString((_a = deleteTarget.value) == null ? void 0 : _a.name), 1),
                        _cache[23] || (_cache[23] = createTextVNode("? ", -1))
                      ]),
                      _cache[24] || (_cache[24] = createBaseVNode("p", { class: "text-caption text-error mt-2" }, "This action cannot be undone.", -1))
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
                      default: withCtx(() => [..._cache[25] || (_cache[25] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      onClick: handleDelete,
                      loading: deleting.value
                    }, {
                      default: withCtx(() => [..._cache[26] || (_cache[26] = [
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
const AgentsPage = /* @__PURE__ */ _export_sfc(_sfc_main, [["__scopeId", "data-v-7a761688"]]);
export {
  AgentsPage as default
};
