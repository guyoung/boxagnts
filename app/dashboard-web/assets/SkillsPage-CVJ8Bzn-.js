import { T as defineStore, K as api, S as defineComponent, ab as useAppStore, a1 as onMounted, P as createElementBlock, M as createBaseVNode, R as createVNode, ai as withCtx, r as VIcon, e as VBtn, z as VRow, aa as unref, N as createBlock, O as createCommentVNode, o as VDialog, a4 as ref, a2 as openBlock, Q as createTextVNode, F as Fragment, a5 as renderList, f as VCard, h as VCardItem, k as VCardTitle, a8 as toDisplayString, i as VCardSubtitle, j as VCardText, g as VCardActions, D as VSwitch, C as VSpacer, n as VCol, H as VTextField, A as VSelect, I as VTextarea, B as VSkeletonLoader, _ as _export_sfc } from "./main-gWZPyuWK.js";
import { u as useCrudOperations } from "./baseCrud-wwuZycIH.js";
const useSkillStore = defineStore("skills", () => {
  const crud = useCrudOperations(
    {
      fetchAll: () => api.getSkills(),
      create: (data) => api.createSkill(data),
      update: (id, data) => api.updateSkill(id, data),
      remove: (id) => api.deleteSkill(id)
    },
    "skills"
  );
  return {
    skills: crud.items,
    loading: crud.loading,
    fetchSkills: crud.fetch,
    addSkill: crud.add,
    updateSkill: crud.update,
    removeSkill: crud.remove
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
  __name: "SkillsPage",
  setup(__props) {
    const skillStore = useSkillStore();
    const appStore = useAppStore();
    const showDialog = ref(false);
    const editingSkill = ref(null);
    const saving = ref(false);
    const deleteDialog = ref(false);
    const deleteTarget = ref(null);
    const deleting = ref(false);
    const availableTypes = [
      "prompt",
      "tool",
      "workflow",
      "template",
      "custom"
    ];
    const defaultForm = () => ({
      name: "",
      description: "",
      type: "prompt",
      config: "",
      enabled: true
    });
    const form = ref(defaultForm());
    function openAddDialog() {
      editingSkill.value = null;
      form.value = defaultForm();
      showDialog.value = true;
    }
    function openEditDialog(skill) {
      editingSkill.value = skill;
      form.value = {
        name: skill.name,
        description: skill.description,
        type: skill.type,
        config: skill.config,
        enabled: skill.enabled
      };
      showDialog.value = true;
    }
    async function handleSave() {
      saving.value = true;
      try {
        if (editingSkill.value) {
          await skillStore.updateSkill(editingSkill.value.id, { ...form.value });
          appStore.showMessage("Skill updated", "success");
        } else {
          await skillStore.addSkill({ ...form.value });
          appStore.showMessage("Skill created", "success");
        }
        showDialog.value = false;
        form.value = defaultForm();
        editingSkill.value = null;
      } catch {
        appStore.showMessage("Failed to save skill", "error");
      } finally {
        saving.value = false;
      }
    }
    function confirmRemove(skill) {
      deleteTarget.value = skill;
      deleteDialog.value = true;
    }
    async function handleDelete() {
      if (!deleteTarget.value) return;
      deleting.value = true;
      try {
        await skillStore.removeSkill(deleteTarget.value.id);
        appStore.showMessage("Skill deleted", "success");
      } catch {
        appStore.showMessage("Failed to delete skill", "error");
      } finally {
        deleting.value = false;
        deleteDialog.value = false;
        deleteTarget.value = null;
      }
    }
    async function handleToggleEnabled(skill, enabled) {
      try {
        await skillStore.updateSkill(skill.id, { enabled });
        appStore.showMessage(enabled ? "Skill enabled" : "Skill disabled", "success");
      } catch {
        appStore.showMessage("Failed to update skill", "error");
      }
    }
    onMounted(() => {
      skillStore.fetchSkills();
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
                createTextVNode("mdi-star", -1)
              ])]),
              _: 1
            }),
            _cache[10] || (_cache[10] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "Skills", -1))
          ]),
          createVNode(VBtn, {
            color: "primary",
            "prepend-icon": "mdi-plus",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[11] || (_cache[11] = [
              createTextVNode(" Add Skill ", -1)
            ])]),
            _: 1
          })
        ]),
        createVNode(VRow, null, {
          default: withCtx(() => [
            (openBlock(true), createElementBlock(Fragment, null, renderList(unref(skillStore).skills, (skill) => {
              return openBlock(), createBlock(VCol, {
                cols: "12",
                md: "6",
                lg: "4",
                key: skill.id
              }, {
                default: withCtx(() => [
                  createVNode(VCard, { class: "fill-height" }, {
                    default: withCtx(() => [
                      createVNode(VCardItem, null, {
                        prepend: withCtx(() => [
                          createVNode(VIcon, {
                            color: skill.enabled ? "primary" : "medium-emphasis",
                            size: "28"
                          }, {
                            default: withCtx(() => [..._cache[12] || (_cache[12] = [
                              createTextVNode(" mdi-star ", -1)
                            ])]),
                            _: 1
                          }, 8, ["color"])
                        ]),
                        default: withCtx(() => [
                          createVNode(VCardTitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(skill.name), 1)
                            ]),
                            _: 2
                          }, 1024),
                          createVNode(VCardSubtitle, null, {
                            default: withCtx(() => [
                              createTextVNode(toDisplayString(skill.type), 1)
                            ]),
                            _: 2
                          }, 1024)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardText, null, {
                        default: withCtx(() => [
                          skill.description ? (openBlock(), createElementBlock("div", _hoisted_3, toDisplayString(skill.description), 1)) : createCommentVNode("", true),
                          skill.config ? (openBlock(), createElementBlock("div", _hoisted_4, [
                            _cache[13] || (_cache[13] = createBaseVNode("div", { class: "text-caption text-medium-emphasis mb-1" }, "Config", -1)),
                            createBaseVNode("div", _hoisted_5, toDisplayString(skill.config), 1)
                          ])) : createCommentVNode("", true)
                        ]),
                        _: 2
                      }, 1024),
                      createVNode(VCardActions, null, {
                        default: withCtx(() => [
                          createVNode(VSwitch, {
                            "model-value": skill.enabled,
                            label: skill.enabled ? "Enabled" : "Disabled",
                            color: "success",
                            density: "compact",
                            "hide-details": "",
                            "onUpdate:modelValue": (v) => handleToggleEnabled(skill, !!v)
                          }, null, 8, ["model-value", "label", "onUpdate:modelValue"]),
                          createVNode(VSpacer),
                          createVNode(VBtn, {
                            variant: "tonal",
                            size: "small",
                            "prepend-icon": "mdi-pencil",
                            onClick: ($event) => openEditDialog(skill)
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
                            onClick: ($event) => confirmRemove(skill)
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
        unref(skillStore).loading ? (openBlock(), createBlock(VSkeletonLoader, {
          key: 0,
          type: "card@3"
        })) : createCommentVNode("", true),
        !unref(skillStore).loading && unref(skillStore).skills.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_6, [
          createVNode(VIcon, {
            size: "64",
            color: "medium-emphasis"
          }, {
            default: withCtx(() => [..._cache[15] || (_cache[15] = [
              createTextVNode("mdi-star-off", -1)
            ])]),
            _: 1
          }),
          _cache[17] || (_cache[17] = createBaseVNode("p", { class: "text-medium-emphasis mt-4" }, "No skills configured", -1)),
          createVNode(VBtn, {
            color: "primary",
            class: "mt-4",
            onClick: openAddDialog
          }, {
            default: withCtx(() => [..._cache[16] || (_cache[16] = [
              createTextVNode("Add Skill", -1)
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
                    createTextVNode(toDisplayString(editingSkill.value ? "Edit Skill" : "Add Skill"), 1)
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
                              placeholder: "code-review",
                              hint: "Unique name for the skill",
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
                              placeholder: "Skill description..."
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
                              placeholder: '{"prompt": "...", "max_tokens": 1000}'
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
                        createTextVNode(toDisplayString(editingSkill.value ? "Update" : "Create"), 1)
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
                    createTextVNode("Delete Skill", -1)
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
const SkillsPage = /* @__PURE__ */ _export_sfc(_sfc_main, [["__scopeId", "data-v-5c3f29e3"]]);
export {
  SkillsPage as default
};
